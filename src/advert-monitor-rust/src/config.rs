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
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DspMonitorConfig {
    pub open: bool,
    #[serde(rename = "allow-overlap-max")]
    pub allow_overlap_max: i64, // s
}
