use std::{fs, path::PathBuf};

use serde::{Deserialize, Serialize};

lazy_static! {
    pub static ref CONFIG: Config = load_config();
}

fn load_config() -> Config {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("Config.yaml");

    let config = fs::read_to_string(path).unwrap();
    serde_yaml::from_str(&config).unwrap()
}

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub name: String,
    pub monitor: MonitorConfig,
}

#[derive(Debug, Clone, Deserialize)]
pub struct MonitorConfig {
    pub cron: String,
    pub dsp: DspMonitorConfig,
    pub ssp: SspMonitorConfig,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SspMonitorConfig {
    pub open: bool,
    #[serde(rename = "me-dsp-id")]
    pub me_dsp_id: i32,
    #[serde(rename = "dev-id-prefix")]
    pub dev_id_prefix: String,
    #[serde(rename = "slot-ids")]
    pub slot_ids: Vec<i32>,
    #[serde(rename = "supe-url")]
    pub supe_url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DspMonitorConfig {
    pub open: bool,
    #[serde(rename = "allow-overlap-max")]
    pub allow_overlap_max: i64, // s
}
