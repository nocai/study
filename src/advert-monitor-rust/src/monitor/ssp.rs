use std::{
    collections::{HashMap, HashSet},
    fs,
    path::PathBuf,
};

use async_std::task::block_on;
use itertools::Itertools;
use reqwest::header::CONTENT_TYPE;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::FromRow;

use crate::{
    config::{self, SspMonitorConfig},
    database::{
        plan::{self, AdConfig, AdConfigRule, Plan},
        supe::{
            media, slot,
            strategy::{self, StrategyConfigDistribution},
        },
        SUPE_POOL,
    },
    error::{self, Error, InternalServer},
    model::{ClientType, DeviceIDType, MediaType, NetworkType, OSType, Size, Version},
    monitor::Monitor,
};

use super::Alarm;

pub struct SspMonitor {}

impl Monitor for SspMonitor {
    fn name(&self) -> &str {
        "SSP"
    }

    fn open(&self) -> bool {
        config::CONFIG.monitor.ssp.open
    }

    fn obtain_data(&self) -> Result<Box<dyn std::any::Any>, crate::error::Error> {
        let config = &config::CONFIG.monitor.ssp;
        let tmpl_args = block_on(find_valid_tmplargs(&config.slot_ids))?;
        log::info!("tmpl_args({}): {:?}", tmpl_args.len(), tmpl_args);

        let tmpls = tmpl_args
            .into_iter()
            .filter_map(|it| Tmpl::try_from(it).ok())
            .collect::<Vec<_>>();
        log::info!("tmlps({}): {:?}", tmpls.len(), tmpls);

        Ok(Box::new(tmpls))
    }

    fn process_data(
        &self,
        data: Box<dyn std::any::Any>,
    ) -> Result<Vec<super::Alarm>, crate::error::Error> {
        let tmpls = data.downcast::<Vec<Tmpl>>().unwrap();
        let config = &config::CONFIG.monitor.ssp;

        let mut alarms = Vec::new();
        for tmpl in tmpls.iter() {
            let result = block_on(ssp(&config, tmpl));
            match result {
                Err(err) => log::error!("{:?}", err),
                Ok(mut als) => alarms.append(&mut als),
            }
        }
        Ok(alarms)
    }
}

async fn ssp(config: &SspMonitorConfig, tmpl: &Tmpl) -> Result<Vec<Alarm>, Error> {
    let slot = slot::get_by_id(tmpl.adslot.id).await?;
    log::info!("sot: {:?}", slot);

    let media = media::get_by_id(slot.media_id).await?;
    log::info!("media: {:?}", media);

    if media.os_type != tmpl.device.os_type {
        log::error!("invalid device.os_type: {:?}", tmpl.device.os_type);
        return error::BadRequest(&format!(
            "invalid device.os_type: {:?}",
            tmpl.device.os_type
        ));
    }

    let strategy = strategy::get_by_slotid_or_mediaid(slot.id, media.id).await?;
    log::info!("strategy: {:?}", strategy);

    let mut alarms = Vec::new();
    for data in strategy.config.data.iter() {
        let mut tmpl = tmpl.clone();
        log::info!("tmpl: {:?}", tmpl);

        if let Ok(app_version) = data.app_version() {
            let app = tmpl.media.app.clone();
            tmpl.media.app = Some(MediaApp {
                version: app_version,
                ..app.unwrap()
            });
            log::info!("after replace version, tmpl: {:?}", tmpl);
        }

        for distribution in data.distributions.iter() {
            match do_ssp(config, &tmpl, distribution).await {
                Ok(mut als) => alarms.append(&mut als),
                Err(err) => log::error!("err: {:?}", err),
            }
        }
    }
    Ok(alarms)
}

async fn do_ssp(
    config: &SspMonitorConfig,
    tmpl: &Tmpl,
    distribution: &StrategyConfigDistribution,
) -> Result<Vec<Alarm>, Error> {
    let dsp_infos = distribution.find_dsp_infos(config.me_dsp_id);
    log::info!("dsp_infos({}): {:?}", dsp_infos.0.len(), dsp_infos);

    let mngs = dsp_infos.mngs().await?;
    log::info!("mngs({}): {:?}", mngs.len(), mngs);

    let mut alarms = Vec::new();
    for mng in mngs.iter() {
        match dsp(config, tmpl, mng.dsp_slot_id.parse().unwrap()).await {
            Err(err) => log::error!("err: {:?}", err),
            Ok(mut als) => alarms.append(&mut als),
        }
    }
    Ok(alarms)
}

async fn dsp(
    config: &SspMonitorConfig,
    tmpl: &Tmpl,
    dsp_slot_id: i32,
) -> Result<Vec<Alarm>, Error> {
    let plans = plan::find_valid_plans().await?;
    log::info!("plans({}): {:?}", plans.len(), plans);

    let plan_results = PlanRuleMatcher::new(tmpl, 336583).match_plans(plans);
    log::info!(
        "plan match results({}): {:?}",
        plan_results.len(),
        plan_results
    );

    let prefix = &config.dev_id_prefix;

    let mut alarms = Vec::new();
    for pr in plan_results.iter() {
        log::info!("plan: {:?}", pr.plan);
        log::info!("tag_rules({}): {:?}", pr.tag_rules.len(), pr.tag_rules);

        let tmpls = pr.gen_tmpls(tmpl, prefix);
        log::info!("tmpls({}): {:?}", tmpls.len(), tmpls);

        for (idx, tmpl) in tmpls.iter().enumerate() {
            log::info!("{}. tmpl: {:?}", idx, tmpl);

            match alarm(config, tmpl, &pr.plan).await {
                Ok(alarm) => alarms.push(alarm),
                Err(err) => log::error!("do_dsp err: {:?}", err),
            }
        }
    }
    todo!()
}

async fn alarm(config: &SspMonitorConfig, tmpl: &Tmpl, plan: &Plan) -> Result<Alarm, Error> {
    let resp = supe_remote(config, tmpl).await?;
    todo!()
}

async fn supe_remote(config: &SspMonitorConfig, tmpl: &Tmpl) -> Result<Value, Error> {
    log::info!("supe_remote, req: {:?}", tmpl);

    let client = reqwest::blocking::Client::new();
    let request = client.post(&config.supe_url).json(tmpl);
    log::debug!("supe_remote, request: {:?}", request);

    let response = request.send()?;
    log::debug!("supe_remote, response: {:?}", response);

    let response = response.json::<Value>()?;
    log::info!("supe_remote, resp: {:?}", response);

    Ok(response)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum PlanRuleMatcherKey {
    SlotID,
    Location,
    OS,
    Scode,
}

impl AsRef<str> for PlanRuleMatcherKey {
    fn as_ref(&self) -> &str {
        match self {
            PlanRuleMatcherKey::SlotID => "slotid",
            PlanRuleMatcherKey::Location => "location",
            PlanRuleMatcherKey::OS => "os",
            PlanRuleMatcherKey::Scode => "scode",
        }
    }
}

#[derive(Debug, Clone)]
struct PlanRuleMatcher(HashMap<PlanRuleMatcherKey, String>);

impl PlanRuleMatcher {
    fn new(tmpl: &Tmpl, dsp_slot_id: i32) -> PlanRuleMatcher {
        let mut map = HashMap::new();
        map.insert(PlanRuleMatcherKey::SlotID, dsp_slot_id.to_string());
        // map.insert("location", v);
        map.insert(PlanRuleMatcherKey::OS, format!("{:?}", tmpl.device.os_type));
        map.insert(PlanRuleMatcherKey::Scode, tmpl.adslot.scode.clone());
        PlanRuleMatcher(map)
    }

    fn match_plans(&self, plans: Vec<Plan>) -> Vec<PlanMatcherResult> {
        log::info!("matcher: {:?}", &self);

        let mut ps = Vec::new();
        for plan in plans.iter() {
            for config in plan.adconfig.0.iter() {
                log::info!("rules: {:?}", config.rules);

                if self.match_adconfig(config) {
                    log::info!("matched. plan_id: {}, rules: {:?}", plan.id, config.rules);

                    ps.push(PlanMatcherResult {
                        plan: plan.clone(),
                        tag_rules: config.tag_rules(),
                    })
                }
            }
        }
        ps
    }

    fn match_adconfig(&self, config: &AdConfig) -> bool {
        for rule in config.rules.iter() {
            if !self.match_adconfig_rule(rule) {
                return false;
            }
        }
        true
    }

    fn match_adconfig_rule(&self, rule: &AdConfigRule) -> bool {
        for (key, value) in self.0.iter() {
            if rule.contains(key.as_ref(), value) {
                return true;
            }
        }
        false
    }
}

#[derive(Debug)]
struct PlanMatcherResult {
    pub plan: Plan,
    pub tag_rules: Vec<AdConfigRule>,
}

impl PlanMatcherResult {
    pub fn gen_tmpls(&self, tmpl: &Tmpl, prefix: &str) -> Vec<Tmpl> {
        if self.tag_rules.is_empty() {
            return vec![tmpl.clone()];
        }

        let device_id_type = tmpl.device.device_id_type();
        let mut vec = Vec::with_capacity(self.tag_rules.capacity());
        for rule in self.tag_rules.iter() {
            let tags = rule.tags();
            for tag in tags.iter() {
                let mut _tmpl = tmpl.clone();
                _tmpl
                    .device
                    .ids
                    .0
                    .insert(Self::device_id(tag, prefix, device_id_type));
                vec.push(_tmpl);
            }
        }
        vec
    }

    fn device_id(tag: &Value, prefix: &str, device_id_type: DeviceIDType) -> DeviceID {
        DeviceID {
            id: format!("{}{}", prefix, tag.to_string()),
            typ: device_id_type,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, FromRow)]
pub struct TmplArg {
    pub slot_id: i32,
    pub os_type: OSType,
}

pub async fn find_valid_tmplargs(slot_ids: &Vec<i32>) -> Result<Vec<TmplArg>, Error> {
    let slot_ids = slot_ids.iter().join(",");
    let args = sqlx::query_as(
        r#"
		select t.id as slot_id, t2.os_type as os_type from adslot t join media t2 on t2.id = t.media
			where t.status = 0 and t.id in (?)"#,
    )
    .bind(slot_ids)
    .fetch_all(&*SUPE_POOL)
    .await?;
    Ok(args)
}

lazy_static! {
    pub static ref TMPL_ANDROID: Tmpl = load_tmpl("TmplAndroid.json");
    pub static ref TMPL_IOS: Tmpl = load_tmpl("TmplIOS.json");
}

fn load_tmpl(path: &str) -> Tmpl {
    let mut path_buf = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path_buf.push(path);

    let config = fs::read_to_string(path_buf).unwrap();
    serde_json::from_str(&config).unwrap()
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Tmpl {
    pub media: Media,
    pub device: Device,
    pub network: Network,
    pub client: Client,
    pub location: Location,
    pub adslot: AdSlot,
}

impl TryFrom<OSType> for Tmpl {
    type Error = Error;

    fn try_from(os_type: OSType) -> Result<Self, Self::Error> {
        match os_type {
            OSType::IOS => Ok(TMPL_IOS.clone()),
            OSType::Android => Ok(TMPL_ANDROID.clone()),
            _ => InternalServer(&format!("invalid os_type: {:?}", os_type)),
        }
    }
}

impl TryFrom<TmplArg> for Tmpl {
    type Error = Error;

    fn try_from(arg: TmplArg) -> Result<Self, Self::Error> {
        let mut tmpl: Tmpl = arg.os_type.try_into()?;
        tmpl.adslot.id = arg.slot_id;
        tmpl.device.os_type = arg.os_type;
        Ok(tmpl)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AdSlot {
    pub id: i32,
    #[serde(rename = "adslot_size")]
    pub size: Size,
    pub need_render: i32,
    pub scode: String,
    pub tags: Vec<KV>,
    pub rules: Vec<KV>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct KV {
    pub key: String,
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Location {
    #[serde(rename = "type")]
    pub typ: i32,
    pub longitude: f32,
    pub latitude: f32,
    pub timestamp: i64,
    pub source: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Client {
    #[serde(rename = "type")]
    pub typ: ClientType,
    #[serde(rename = "client_version")]
    pub version: Version,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Network {
    #[serde(rename = "type")]
    pub typ: NetworkType,
    pub imsi: Option<String>,
    pub cellular_id: Option<String>,
    pub cellular_operator: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Media {
    #[serde(rename = "type")]
    pub typ: MediaType,
    pub app: Option<MediaApp>,
    pub site: Option<MediaSite>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MediaApp {
    pub package_name: String,
    pub channel_id: String,
    #[serde(rename = "app_version")]
    pub version: Version,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MediaSite {
    pub domain: String,
    pub url: String,
    pub title: String,
    pub keywords: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Device {
    #[serde(rename = "type")]
    pub typ: i32,
    pub ids: DeviceIDs,
    pub os_type: OSType,
    pub os_version: Version,
    pub brand: String,
    pub model: String,
    pub screen_size: Size,
    pub screen_density: f32,
    pub dzh_user: String,
}

impl Device {
    pub fn device_id_type(&self) -> DeviceIDType {
        match self.os_type {
            OSType::Android => DeviceIDType::IMEI,
            OSType::IOS => DeviceIDType::IDFA,
            _ => DeviceIDType::default(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DeviceIDs(pub HashSet<DeviceID>);

#[derive(Debug, Clone, Serialize, Deserialize, Hash, PartialEq, Eq, Default)]
pub struct DeviceID {
    pub id: String,
    #[serde(rename = "type")]
    pub typ: DeviceIDType,
}
