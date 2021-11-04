use crate::common::error::Error;
use log::info;
use sqlx::MySqlPool;

use sqlx::mysql::MySqlPoolOptions;
use std::env;
use std::fs;
use std::path::PathBuf;

use super::config::Config;

#[derive(Clone)]
pub struct AppState {
    pub pool: MySqlPool,
    pub config: Config,
}

impl AppState {
    pub async fn new() -> Result<AppState, Error> {
        let config_path = AppState::default_config_path();
        let config = fs::read_to_string(config_path)?;
        // info!("config: {:?}", config);

        let config: Config = serde_yaml::from_str(&config)?;
        info!("config: {:?}", config);

        let data_base_url = env::var("DATABASE_URL")?;
        let pool = MySqlPoolOptions::new()
            .max_connections(1)
            // .after_connect(|conn| Box::pin(async move {
            //     conn.execute("SET time_zone = '+08:00';").await?;
            //     Ok(())
            // }))
            .connect(&data_base_url)
            .await?;
        Ok(AppState {
            pool,
            config: config.clone(),
        })
    }

    fn default_config_path() -> PathBuf {
        let mut config_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        config_path.push("src");
        config_path.push("config.yaml");
        config_path
    }
}
