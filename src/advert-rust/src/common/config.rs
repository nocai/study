use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub version: String,
    pub dsp: DspConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DspConfig {
    pub timeout: u64,
    pub me_dsp: MeDspConfig,
    pub wan_hui_dsp: WanHuiDspConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MeDspConfig {
    pub location: Location,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Location {
    pub v1: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WanHuiDspConfig {
    pub adspace_id: String,
    pub app_id: String,
    pub api_version: String,
    pub key: String,
    pub iv: String,
    pub location: String,
    pub user_id: u64,
}
