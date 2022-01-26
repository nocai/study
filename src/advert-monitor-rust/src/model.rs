use serde::{Deserialize, Serialize};
use serde_repr::Deserialize_repr;
use sqlx::Type;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Type)]
#[repr(i32)]
pub enum Status {
    Invalid = 1,
    Valid = 0,
}

impl Default for Status {
    fn default() -> Self {
        Status::Invalid
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Type)]
#[repr(i32)]
pub enum BidType {
    CPT = 1,
    CPM = 2,
    CPC = 3,
}

impl Default for BidType {
    fn default() -> Self {
        BidType::CPT
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize_repr, Type, PartialEq)]
#[repr(i32)]
pub enum OSType {
    Unknown,
    Android = 1,
    IOS = 2,
    WP = 3,
}

impl Default for OSType {
    fn default() -> Self {
        OSType::Unknown
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Version {
    pub major: u32,
    pub minor: u32,
    pub micro: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Size {
    pub width: i32,
    pub height: i32,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize_repr, Type)]
#[repr(i32)]
pub enum MediaType {
    Unknown,
    App = 1,
    Wap = 2,
}

impl Default for MediaType {
    fn default() -> Self {
        MediaType::Unknown
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize_repr, Type)]
#[repr(i32)]
pub enum NetworkType {
    Unknown = 0,
    WIFI = 1,
    G2 = 2,
    G3 = 3,
    G4 = 4,
}

impl Default for NetworkType {
    fn default() -> Self {
        NetworkType::Unknown
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize_repr, Type)]
#[repr(i32)]
pub enum ClientType {
    NativeSDK = 1,
    JSSDK = 2,
    ClientOpenAPI = 3,
    ServerOpenAP = 4,
}

impl Default for ClientType {
    fn default() -> Self {
        ClientType::NativeSDK
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize_repr, Type, Hash, PartialEq, Eq)]
#[repr(i32)]
pub enum DeviceIDType {
    Unknown = 0,
    IMEI = 1,
    MAC = 2,
    IDFA = 3,
    AndroidID = 4,
    IDFV = 5,
    OpenUDID = 6,
    LSCookie = 7,
    OAID = 8,
}

impl Default for DeviceIDType {
    fn default() -> Self {
        DeviceIDType::Unknown
    }
}
