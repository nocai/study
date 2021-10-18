pub mod dsp;
pub mod media;
pub mod slot;
pub mod strategy;

use chrono::{DateTime, Local};
use derive_more::Display;
use log::info;
use serde::{Deserialize, Serialize};
use sqlx::MySqlPool;
use std::{collections::HashMap, fmt::Display};

use crate::{
    common::{app_state::AppState, error::Error},
    modules::{random_by_weight, ssp::strategy::Strategy},
};

use self::{
    media::database::MediaType,
    strategy::{BindDsp, TriggerRule, TriggerTarget},
};

use super::enums::OS;

#[derive(Debug, Deserialize)]
pub struct Request {
    pub slot: Slot,
    pub media: Media,
    pub device: Device,
    pub network: Network,
    pub location: Location,
    // 传递其它值
    #[serde(skip_deserializing)]
    pub ip: String,
    #[serde(skip_deserializing)]
    pub user_agent: String,
    #[serde(skip_deserializing)]
    pub host: String,
}

#[derive(Debug, Deserialize)]
pub struct Slot {
    pub id: u64,
    pub width: u32,
    pub height: u32,
}

#[derive(Debug, Deserialize)]
pub struct Location {
    pub longitude: Option<f64>,
    pub latitude: Option<f64>,
    pub timestamp: Option<DateTime<Local>>,
    pub source: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Network {
    pub network_type: NetworkType,
    pub imsi: String,
    pub cellular_id: String,
    pub cellular_operator: String,
}

#[derive(Debug, Clone, Display, PartialEq, Eq, Deserialize)]
pub enum NetworkType {
    Unknown,
    WIFI,
    G2,
    G3,
    G4,
    G5,
}

#[derive(Debug, Deserialize)]
pub struct Device {
    pub os_type: OS,
    pub os_version: Version,
    pub brand: String,
    pub model: String,
    pub screen_width: u32,
    pub screen_height: u32,
    pub screen_density: f32,
    pub ids: Vec<DeviceID>,
}

#[derive(Debug, Clone, Deserialize)]
pub enum DeviceID {
    Unknown,
    IMEI { id: String, md5: bool },
    MAC { id: String, md5: bool },
    IDFA { id: String, md5: bool },
    AndroidID { id: String, md5: bool },
    IDFV { id: String, md5: bool },
    OpenUDID { id: String, md5: bool },
    LSCookie { id: String, md5: bool },
    OAID { id: String, md5: bool },
}

#[derive(Debug, Deserialize)]
pub enum Media {
    App {
        package_name: String,
        version: Version,
    },
    Wap {
        domain: String,
        url: String,
        title: String,
        keywords: Vec<String>,
    },
}

#[derive(Debug, Clone, Copy, Deserialize)]
pub struct Version {
    pub major: u8,
    pub minor: u8,
    pub micro: u8,
}

impl Version {
    pub fn major_minor(&self) -> String {
        format!("{}.{}", self.major, self.minor)
    }
}

impl Display for Version {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}.{}", self.major, self.minor, self.micro)
    }
}

use actix_web::{
    http::{header, HeaderName, StatusCode},
    post,
    web::{Data, Json},
    HttpRequest, HttpResponse,
};

#[post("/ssp")]
pub async fn ssp(
    state: Data<AppState>,
    req: Json<Request>,
    req2: HttpRequest,
) -> Result<HttpResponse, Error> {
    let mut req = req.into_inner();
    fill_request_from_header(&mut req, req2);
    info!("req: {:?}", req);

    let r = do_ssp(&state.pool, &req).await?;
    Ok(HttpResponse::Ok().json(r))
    // Ok(HttpResponse::new(StatusCode::OK))
}

fn fill_request_from_header(req: &mut Request, req2: HttpRequest) {
    req.host = get_from_request_header(&req2, header::HOST);
    req.user_agent = get_from_request_header(&req2, header::USER_AGENT);
    req.ip = req2
        .peer_addr()
        .map(|v| v.ip().to_string())
        .unwrap_or_default();
}

fn get_from_request_header(req: &HttpRequest, header: HeaderName) -> String {
    let result = req
        .headers()
        .get(header)
        .map(|v| v.to_str().unwrap_or_default())
        .unwrap_or_default()
        .to_string();
    result
}

#[derive(Debug, Serialize)]
pub struct Response {}
async fn do_ssp(pool: &MySqlPool, req: &Request) -> Result<Option<Response>, Error> {
    let slot = slot::get_by_id(pool, req.slot.id)
        .await?
        .ok_or(Error::BadRequest(
            400,
            format!("invalid slot.id: {}", req.slot.id).as_str(),
        ))?;

    let media = media::get_by_id(pool, slot.media_id)
        .await?
        .ok_or(Error::UnprocessableEntity(
            422,
            &format!("invalid media_id: {} from slot: {:?}", slot.media_id, slot),
        ))?;

    let strategy = strategy::get_by_slot_id_and_media_id(pool, slot.id, media.id)
        .await?
        .ok_or(Error::UnprocessableEntity(
            422,
            &format!(
                "invalid slot_id: {}, media_id: {} from slot: {:?} media: {:?}",
                slot.id, media.id, slot, media
            ),
        ))?;

    let targets = match_by_strategy(pool, &*req, strategy).await?;
    if targets.is_empty() {
        info!("targets is empty. return");
        return Ok(None);
    }

    let target = random_by_weight(&targets);
    info!("the final targes: {:?}", target);

    let dsp = find_dsp_by_target(pool, target).await?;
    info!("the final dsp: {:?}", &dsp);

    Ok(Some(Response {}))
}

async fn find_dsp_by_target(pool: &MySqlPool, target: &TriggerTarget) -> Result<BindDsp, Error> {
    let dsps = strategy::find_dsps_by_ids(pool, target.group1()?).await?;
    if !dsps.is_empty() {
        let dsp = random_by_weight(&dsps);
        info!("the final dsp by group1: {:?}", dsp);
        return Ok(dsp.clone());
    }

    let dsps = strategy::find_dsps_by_ids(pool, target.group2()?).await?;
    if !dsps.is_empty() {
        let dsp = random_by_weight(&dsps);
        info!("the final dsp by group2: {:?}", dsp);
        return Ok(dsp.clone());
    }
    Err(Error::UnprocessableEntity(
        422,
        &format!("invalid target: {:?}", target),
    ))
}

async fn match_by_strategy(
    pool: &MySqlPool,
    req: &Request,
    strategy: Strategy,
) -> Result<Vec<TriggerTarget>, Error> {
    let configs = strategy::find_configs_by_strategy_id(pool, strategy.id).await?;
    if configs.is_empty() {
        info!("strategy_configs is empty! return");
        return Ok(Vec::new());
    }

    let match_rule = MatchRule::new(&*req);
    info!("match_rule: {:?}", match_rule);

    for config in configs.iter() {
        let rules = strategy::find_rules_by_config_id(pool, config.id).await?;
        if match_rule.match_trigger_rules(&rules) {
            return strategy::find_trigger_targets_by_config_id(pool, config.id).await;
        }
    }

    info!("doesn't match any config. len: {}", configs.len());
    Ok(Vec::new())
}

#[derive(Debug)]
struct MatchRule<'a>(HashMap<&'a str, String>);

impl<'a> MatchRule<'a> {
    fn new(req: &Request) -> Self {
        let mut map = HashMap::new();
        map.insert("os_type", req.device.os_type.to_string());
        map.insert("network_type", req.network.network_type.to_string());
        map.insert(
            "media_type",
            match req.media {
                Media::App { .. } => MediaType::APP.to_string(),
                Media::Wap { .. } => MediaType::WAP.to_string(),
            },
        );
        for device_id in req.device.ids.iter() {
            match &device_id {
                DeviceID::IMEI { id, .. } => {
                    map.insert("imei", id.clone());
                }
                DeviceID::IDFA { id, .. } => {
                    map.insert("idfa", id.clone());
                }
                DeviceID::AndroidID { id, .. } => {
                    map.insert("android_id", id.clone());
                }
                DeviceID::IDFV { id, .. } => {
                    map.insert("idfv", id.clone());
                }
                _ => {}
            }
        }

        if let Media::App {
            package_name,
            version,
        } = &req.media
        {
            map.insert("package_name", package_name.clone());
            map.insert("app_version", version.to_string());
        }

        map.insert("ip", req.ip.clone());

        // todo:
        map.insert("browser", String::default());
        map.insert("country", String::default());
        map.insert("province", String::default());
        map.insert("city", String::default());
        MatchRule(map)
    }

    fn match_trigger_rules(&self, rules: &Vec<TriggerRule>) -> bool {
        for rule in rules.iter() {
            if !self.match_trigger_rule(rule) {
                return false;
            }
        }
        info!("matched all rules. len: {}", rules.len());
        true
    }

    fn match_trigger_rule(&self, rule: &TriggerRule) -> bool {
        let matched = self
            .0
            .get(&*rule.key)
            .map_or(false, |value| rule.contains(value).unwrap_or_default());
        info!("matched rule: {}, rule: {:?}, ", matched, rule);
        matched
    }
}
