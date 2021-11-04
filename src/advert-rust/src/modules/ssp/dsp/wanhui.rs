use std::{
    convert::{TryFrom, TryInto},
    str::FromStr,
};

use chrono::Local;
use derive_more::Display;
use log::info;
use reqwest::Url;
use serde::{Deserialize, Serialize};
use serde_repr::*;

use crate::{
    common::{app_state::AppState, config::WanHuiDspConfig, error::Error},
    modules::{
        enums::{AdType, OS},
        ssp::{self, dsp_slot::DspSlot, Location},
    },
};

use super::Dsp;

pub struct WanHuiDsp<'a> {
    config: &'a WanHuiDspConfig,
}

impl<'a> Dsp for WanHuiDsp<'a> {
    fn new_reqwest_request(
        &self,
        req: &ssp::Request,
        slot: &DspSlot,
    ) -> Result<reqwest::blocking::Request, Error> {
        let body = self.new_request(req, slot)?;
        info!("req: {:?}", body);

        let url = Url::parse(&self.config.location).unwrap();
        info!("req url: {:?}", url.to_string());

        let req2 = reqwest::blocking::Client::new().post(url).json(&body);
        Ok(req2.build()?)
        // Err(Error::BadRequest(400, "message"))
    }

    fn handle_reqwest_response(
        &self,
        resp: reqwest::blocking::Response,
    ) -> Result<Vec<ssp::Ad>, Error> {
        let resp: Response = resp.json()?;
        if let Some(code) = resp.error_code {
            if code > 0 {
                return Err(Error::InternalServerError(
                    code as u32,
                    resp.error_msg.unwrap_or_default().as_str(),
                ));
            }
        }

        Ok(resp.into())

        // let a = resp
        //     .ads
        //     .unwrap_or_default()
        //     .into_iter()
        //     .map(|v| {
        //         let a: Result<Ad, Error> = v.try_into();
        //     })
        //     .collect();
        // let a = resp.ads.map(|ads| {
        //     ads.into_iter()
        //         .map(|v| v.try_into())
        //         .collect::<Result<Vec<ssp::Ad>, Error>>()
        // });
        // Ok(Vec::new())
    }

    fn handle_reqwest_response_error(
        &self,
        resp: reqwest::blocking::Response,
    ) -> Result<Vec<ssp::Ad>, Error> {
        let status = resp.status();

        let resp: Response = resp.json()?;
        info!("status: {:?}, resp: {:?}", status, resp);

        Err(Error::new(
            resp.error_code.unwrap_or_default() as u32,
            resp.error_msg.unwrap_or_default().as_str(),
            status,
        ))
    }
}

const DEFAULT_END_TYPE: u8 = 1;

impl<'a> WanHuiDsp<'a> {
    pub fn new(state: &'a AppState) -> Self {
        Self {
            config: &state.config.dsp.wan_hui_dsp,
        }
    }

    pub fn new_request(&self, req: &ssp::Request, slot: &DspSlot) -> Result<Request, Error> {
        let mut app: App = req.media.clone().try_into()?;
        app.app_id = self.config.app_id.clone();
        app.package_name = slot
            .package
            .as_ref()
            .or(app.package_name.as_ref())
            .map(|v| v.to_string());

        let mut adspace: Adspace = req.slot.clone().try_into()?;
        adspace.adspace_id = self.config.adspace_id.clone();
        adspace.adspace_type = slot
            .slot_type
            .as_ref()
            .map(|v| AdspaceType::from_str(v).unwrap_or_default())
            .unwrap_or_default();

        Ok(Request {
            bid: self.request_bid(req),
            api_version: self.config.api_version.clone(),
            end_type: DEFAULT_END_TYPE,
            ua: req.user_agent.clone(),
            app,
            device: req.device.clone().into(),
            net_work: req.network.clone().into(),
            gps: Some(req.location.clone().into()),
            adspaces: vec![adspace],
            debug: false,
            browser_ua: req.user_agent.clone(),
        })
    }

    // buildBid 请求的唯一 id，32 个字节的 字符串，由媒体端生成。生 成规则:MD5(设备号+APP 包名+时间戳(毫秒))
    fn request_bid(&self, req: &ssp::Request) -> String {
        let device_no = "";
        let package_name = match &req.media {
            ssp::Media::App { package_name, .. } => package_name.clone(),
            _ => String::new(),
        };
        let timestamp = Local::now().timestamp_subsec_millis();

        // 拼接后md5
        let s = format!("{:?}{:?}{:?}", device_no, package_name, timestamp);
        format!("{:x}", md5::compute(s))
    }
}

#[derive(Debug, Serialize)]
pub struct Request {
    pub bid: String,
    pub api_version: String,
    pub end_type: u8,
    pub ua: String,
    pub app: App,
    pub device: Device,
    pub net_work: Network,
    pub gps: Option<GPS>,
    pub adspaces: Vec<Adspace>,
    pub debug: bool,
    pub browser_ua: String,
}

#[derive(Debug, Serialize)]
pub struct App {
    pub app_id: String,
    pub app_name: Option<String>,
    pub package_name: Option<String>,
    pub app_keywords: Option<String>,
    pub app_version: String,
}

impl TryFrom<ssp::Media> for App {
    type Error = Error;
    fn try_from(media: ssp::Media) -> Result<App, Self::Error> {
        match media {
            ssp::Media::App {
                package_name,
                version,
            } => {
                return Ok(Self {
                    app_name: Some(String::from("大智慧")),
                    app_keywords: None,
                    app_version: version.to_string(),
                    app_id: Default::default(),
                    package_name: Some(package_name),
                });
            }
            _ => Err(Error::InternalServerError(500, "not app")),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct Device {
    // DeviceId 对象列表，建议尽可能多 填写几个 IMEI for Android IDFA、IDFV for IOS
    pub device_id: Vec<DeviceID>,
    // 操作系统类型
    pub os_type: OSType,
    // 操作系统版本号
    pub os_version: String,
    // 设备品牌
    pub brand: String,
    // 设备型号
    pub model: String,
    // 设备类型
    pub device_type: DeviceType,
    // 设备设置语言
    pub language: String,
    // 设备屏幕的宽度，以像素为单位，与密度无关
    pub screen_width: u32,
    // 设备屏幕的高度，以像素为单位，与密度无关
    pub screen_height: u32,
    // 屏幕密度(DPI 与 160 的比值)
    pub screen_density: f32,
    // 屏幕朝向
    pub screen_orientation: ScreenOrientation,
    // 当前设备是否越狱(或 ROOT)
    pub jail_breaked: Option<bool>,
    // 系统 Api 版本
    pub os_api_level: Option<String>,
}

impl From<ssp::Device> for Device {
    fn from(dev: ssp::Device) -> Self {
        Self {
            device_id: dev.ids.into_iter().map(|id| id.into()).collect(),
            os_type: dev.os.into(),
            os_version: dev.os_version.major_minor(),
            brand: dev.brand,
            model: dev.model,
            device_type: dev.device_type.into(),
            language: Default::default(), // Fix:
            screen_width: dev.screen_width,
            screen_height: dev.screen_height,
            screen_density: Default::default(),             // Fix:
            screen_orientation: ScreenOrientation::Unknown, // Fix:
            jail_breaked: None,
            os_api_level: None,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct DeviceID {
    pub device_id: String,
    pub device_id_type: DeviceIDType,
}

impl From<ssp::DeviceID> for DeviceID {
    fn from(device_id: ssp::DeviceID) -> Self {
        match device_id {
            ssp::DeviceID::IMEI { id, .. } => Self {
                device_id: id,
                device_id_type: DeviceIDType::IMEI,
            },
            ssp::DeviceID::IDFA { id, .. } => Self {
                device_id: id,
                device_id_type: DeviceIDType::IDFA,
            },
            ssp::DeviceID::MAC { id, .. } => Self {
                device_id: id,
                device_id_type: DeviceIDType::MAC,
            },
            ssp::DeviceID::IDFV { id, .. } => Self {
                device_id: id,
                device_id_type: DeviceIDType::IDFV,
            },
            ssp::DeviceID::OAID { id, .. } => Self {
                device_id: id,
                device_id_type: DeviceIDType::OAID,
            },
            _ => Self {
                device_id: String::new(),
                device_id_type: DeviceIDType::Unknow,
            },
        }
    }
}

#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug)]
#[repr(u8)]
pub enum DeviceIDType {
    Unknow = 0,
    IMEI = 1,
    IDFA = 2,
    AAID = 3,
    MAC = 4,
    IDFV = 5,
    M2ID = 6,
    SerialID = 7,
    ImeiSha1 = 9,
    OAID = 10,
}

#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug)]
#[repr(u8)]
pub enum ScreenOrientation {
    Unknown = 0,
    Portrait = 1,
    // 竖屏
    Landscape = 2, // 横屏
}

#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug)]
#[repr(u8)]
pub enum DeviceType {
    Unknown = 0,
    Tablet = 1,
    Phone = 2,
    PC = 3,
}

impl From<ssp::DeviceType> for DeviceType {
    fn from(device_type: ssp::DeviceType) -> Self {
        match device_type {
            ssp::DeviceType::Tablet => Self::Tablet,
            ssp::DeviceType::Phone => Self::Phone,
            ssp::DeviceType::PC => Self::PC,
            _ => Self::Unknown,
        }
    }
}

#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug)]
#[repr(u8)]
pub enum OSType {
    Unknown = 0,
    IOS = 1,
    Android = 2,
    WP = 3,
}

impl From<OS> for OSType {
    fn from(os: OS) -> Self {
        match os {
            OS::Android => Self::Android,
            OS::IOS => Self::IOS,
            OS::WindowsPhone => Self::WP,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct Network {
    pub ip: Option<String>,
    // 用户的 ip 地址，目前客户端传 值为空; 服务端从请求头获取 IP，没有采用该值。
    pub ipv6: Option<String>,
    // 用户的ipv6地址
    pub network_type: NetworkType,
    // 网络类型
    pub carrier_id: CarrierID,
    // 运营商编码
    pub real_carrier_id: Option<String>,
    // 运营商 ID *carrier_id 是 real_carrier_id 进 行转换后的，不全
    pub cellular_id: Option<CellularID>, // 基站 ID，用于快速用户定位
}

impl From<ssp::Network> for Network {
    fn from(network: ssp::Network) -> Self {
        let carrier_id = network.carrier.into();
        Self {
            ip: network.ip,
            ipv6: network.ipv6,
            network_type: network.network_type.into(),
            real_carrier_id: None,
            cellular_id: network.cellular_id.map(|c| {
                let mut cc: CellularID = TryInto::try_into(c).unwrap_or_default();
                cc.set_mnc(&carrier_id);
                cc
            }),
            carrier_id,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct CellularID {
    pub mcc: i32,
    pub mnc: i32,
    pub lac: i32,
    pub cid: i32,
}

impl Default for CellularID {
    fn default() -> Self {
        Self {
            mcc: 460,
            mnc: Default::default(),
            lac: Default::default(),
            cid: Default::default(),
        }
    }
}

impl TryFrom<String> for CellularID {
    type Error = Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let mut cellular_id = Self::default();
        cellular_id.cid = value.parse()?;
        Ok(cellular_id)
    }
}

impl CellularID {
    fn set_mnc(&mut self, cid: &CarrierID) {
        self.mnc = match cid {
            CarrierID::ChinaMobile => 0,
            CarrierID::Unicom => 1,
            _ => Default::default(),
        }
    }
}

#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug)]
#[repr(u8)]
pub enum CarrierID {
    Unknown = 0,
    ChinaMobile = 1,
    // 移动
    ChinaTelecom = 2,
    // 电信
    Unicom = 3, // 联通
}

impl From<ssp::Carrier> for CarrierID {
    fn from(carrier: ssp::Carrier) -> Self {
        match carrier {
            ssp::Carrier::ChinaMobile => Self::ChinaMobile,
            ssp::Carrier::ChinaTelecom => Self::ChinaTelecom,
            ssp::Carrier::ChinaUnicom => Self::Unicom,
            _ => Self::Unknown,
        }
    }
}

#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug)]
#[repr(u8)]
pub enum NetworkType {
    Unknown = 0,
    WIFI = 1,
    _2G = 2,
    _3G = 3,
    _4G = 4,
    _5G = 5,
}

impl From<ssp::NetworkType> for NetworkType {
    fn from(network_type: ssp::NetworkType) -> Self {
        match network_type {
            ssp::NetworkType::WIFI => Self::WIFI,
            ssp::NetworkType::G2 => Self::_2G,
            ssp::NetworkType::G3 => Self::_3G,
            ssp::NetworkType::G4 => Self::_4G,
            ssp::NetworkType::G5 => Self::_5G,
            _ => Self::Unknown,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct GPS {
    pub gps_type: GPSType,
    pub longitude: f64,
    pub latitude: f64,
    pub timestamp: i64,
}

impl From<ssp::Location> for GPS {
    fn from(loc: Location) -> Self {
        Self {
            gps_type: GPSType::WGS84,
            longitude: loc.longitude.unwrap_or_default(),
            latitude: loc.latitude.unwrap_or_default(),
            timestamp: loc.timestamp.unwrap_or(Local::now()).timestamp(),
        }
    }
}

#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug)]
#[repr(u8)]
pub enum GPSType {
    WGS84 = 1,
    // 全球卫星定位系 统坐标系
    GCJ02 = 2,
    // 国家测绘局坐标 系
    BD09 = 3, // 百度坐标系
}

#[derive(Debug, Serialize)]
pub struct Adspace {
    pub adspace_id: String,
    pub adspace_type: AdspaceType,
    pub adspace_position: Option<AdspacePosition>,
    pub allow_html: bool,
    pub width: u32,
    pub height: u32,
    pub impression_num: u32,
    pub open_type: OpenType,
    pub interaction_type: Vec<InteractionType>,
    pub asset: Option<Asset>,
    pub impression_time: Option<u64>,
}

impl TryFrom<ssp::Slot> for Adspace {
    type Error = Error;

    fn try_from(slot: ssp::Slot) -> Result<Self, Self::Error> {
        Ok(Self {
            adspace_id: Default::default(),
            adspace_type: Default::default(),
            adspace_position: None,
            allow_html: false,
            width: slot.width,
            height: slot.height,
            impression_num: 1,
            open_type: OpenType::All,
            interaction_type: vec![InteractionType::Any],
            asset: None,
            impression_time: None,
        })
    }
}

// 广告位类型
// {
// BANNER = 1; //横幅广告
// OPENING = 2; //开屏广告
// INTERSTITIAL = 3; //插屏广告
// NATIVE = 4; //信息流广告
// }
#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug, Display)]
#[repr(u8)]
pub enum AdspaceType {
    Banner = 1,
    Opening = 2,
    Interstitial = 3,
    Native = 4,
}

impl FromStr for AdspaceType {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let ats = vec![
            Self::Banner,
            Self::Opening,
            Self::Interstitial,
            Self::Native,
        ];
        for at in ats {
            if at.to_string() == s {
                info!("from_str by name: {:?}", s);
                return Ok(at);
            }
        }
        info!("from_str by serde_json");
        Ok(serde_json::from_str(s)?)
    }
}

impl Default for AdspaceType {
    fn default() -> Self {
        Self::Banner
    }
}

#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug)]
#[repr(u8)]
pub enum AdspacePosition {
    Unknown = 0,
    FirstPosition = 1,
    Others = 2,
}

#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug, Clone)]
#[repr(u8)]
pub enum OpenType {
    All = 0,
    Inner = 1,
    Outer = 2,
}

// 广告位允许的交互类型
// {
// ANY = 0; //任何一种
// NO_INTERACTION = 1;
// BROWSE= 2; //浏览
// DOWNLOAD =3; //下载
// VIDEO = 7; //video
// DEEPLINK = 8; //直达
// }
#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug, Clone)]
#[repr(u8)]
pub enum InteractionType {
    Any = 0,
    NoInteraction = 1,
    Browser = 2,
    Download = 3,
    Video = 7,
    Deeplink = 8,
}

impl Default for InteractionType {
    fn default() -> Self {
        Self::Any
    }
}

impl From<InteractionType> for ssp::InteractionType {
    fn from(val: InteractionType) -> Self {
        match val {
            InteractionType::NoInteraction => Self::NoInteraction,
            InteractionType::Download => Self::Download,
            InteractionType::Deeplink => Self::DeepLink,
            _ => Self::Surfing,
        }
    }
}

// 对当前广告位所需物料有明确要求 时，可以通过该字段指定物料被必 备字段。
// Enum Asset {
// TITLE = 1; //推广标题
// TEXT = 2; //推广摘要
// ICON_IMAGE = 3; //广告 ICON 图 标
// MAIN_IMAGE = 4; //广告图片
// }
#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug)]
#[repr(u8)]
pub enum Asset {
    Title = 1,
    Text = 2,
    IconImage = 3,
    MainImage = 4,
}

// ====================== response ==============================================

#[derive(Debug, Clone, Deserialize)]
pub struct Response {
    pub bid: String,
    pub ads: Option<Vec<Ad>>,
    pub error_code: Option<i64>,
    pub error_msg: Option<String>,
}

impl From<Response> for Vec<ssp::Ad> {
    fn from(value: Response) -> Self {
        let ads = value.ads.unwrap_or_default();

        ads.into_iter()
            .filter_map(|ad| ad.try_into().ok())
            .collect()

        // Err(Error::BadRequest(400, "message"))
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct Ad {
    pub adspace_id: String,
    pub creative: Vec<Creative>,
}

impl TryFrom<Ad> for ssp::Ad {
    type Error = Error;
    fn try_from(val: Ad) -> Result<Self, Self::Error> {
        if val.creative.is_empty() {
            return Err(Error::InternalServerError(500, "require creative"));
        }

        let creative = val.creative[0].to_owned();
        let mut ad: ssp::Ad = creative.into();
        ad.ad_id = val.adspace_id;
        Ok(ad)
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct Creative {
    pub banner_id: String,
    pub open_type: OpenType,
    pub interaction_type: Option<InteractionType>,
    pub interacton_object: Option<InteractionObject>,
    pub adm_type: AdmType,
    pub adm: Adm,
    pub event_track: Vec<EventTrack>,
    pub download: Option<Download>,
}

impl From<Creative> for ssp::Ad {
    fn from(val: Creative) -> Self {
        let mut ad: ssp::Ad = val.adm.into();

        ad.adtype = val.adm_type.into();
        ad.interaction_type = val.interaction_type.unwrap_or_default().into();
        if let Some(inter) = val.interacton_object {
            ad.deeplink_url = inter.deeplink;
            ad.click_url = Some(inter.url);
        }

        ad
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct Download {
    pub md5: String,
    pub url: String,
    pub package_name: Option<String>,
    pub size: Option<i32>,
    pub app_version: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct EventTrack {
    pub event_type: EventType,
    pub notify_url: String,
}

#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug, Clone)]
#[repr(u8)]
pub enum EventType {
    Pattern = 0,
    // TODO: others
}

#[derive(Debug, Clone, Deserialize)]
pub struct Adm {
    pub source: String,
    pub nativ: Native,
    pub style: AdmStyle,
}

impl From<Adm> for ssp::Ad {
    fn from(val: Adm) -> Self {
        val.nativ.into()
    }
}

#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug, Clone)]
#[repr(u8)]
pub enum AdmStyle {
    OpenscreenDef = 01,     // 开屏广告
    InfoflowOnePic = 11,    // 信息流大图广告
    InfoFlowThreePic = 12,  // 信息流三图广告
    InfoFlowLeftPic = 13,   // 左图
    InfoFlowRightPic = 14,  // 右图
    InfoFlowTopPic = 15,    // 顶图
    InfoFlowBottomPic = 16, // 底图
    BannerDef = 31,         // 横幅
}

#[derive(Debug, Clone, Deserialize)]
pub struct Native {
    pub img: Img,
    pub title: Title,
    pub desc: Option<String>,
    pub source: Option<String>,
    #[serde(rename = "buttonTxt")]
    pub button_txt: Option<String>,
    #[serde(rename = "imgList")]
    pub img_list: Option<Vec<Img>>,
}

impl From<Native> for ssp::Ad {
    fn from(val: Native) -> Self {
        let mut ad = Self::default();

        ad.image = Some(val.img.into());
        ad.images = val
            .img_list
            .map(|vec| vec.into_iter().map(|img| img.into()).collect());
        ad
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct Img {
    pub url: String,
    pub width: Option<u32>,
    pub height: Option<u32>,
    pub md5: String,
    pub img_type: ImgType,
}

impl From<Img> for ssp::Image {
    fn from(val: Img) -> Self {
        Self {
            url: val.url,
            width: val.width.unwrap_or_default(),
            height: val.height.unwrap_or_default(),
        }
    }
}

#[derive(Deserialize_repr, PartialEq, Debug, Clone)]
#[repr(u8)]
pub enum ImgType {
    Master = 1, // 主图
    Icon = 2,   // icon图
    Button = 3, // button图
}

#[derive(Debug, Clone, Deserialize)]
pub struct Title {
    pub text: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct InteractionObject {
    pub url: String,
    pub deeplink: Option<String>,
}

#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug, Clone)]
#[repr(u8)]
pub enum AdmType {
    PIC = 0,
    HTML = 1,
    NATIVE = 3,
}

impl From<AdmType> for AdType {
    fn from(_: AdmType) -> Self {
        // TODO:
        AdType::Image
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test1() {
        let assert = Asset::IconImage;
        let j = serde_json::to_string(&assert).unwrap();
        println!("{:?}", j);
        let assert: Asset = serde_json::from_str(&j).unwrap();
        println!("{:?}", assert);

        let at = AdspaceType::from_str(&AdspaceType::Interstitial.to_string()).unwrap();
        println!("{:?}", at);
    }
}
