use crate::common::error::Error;
use sqlx::{Executor, MySqlPool};

use sqlx::mysql::MySqlPoolOptions;
use std::env;

#[derive(Clone)]
pub struct AppState {
    pub pool: MySqlPool,
}

impl AppState {
    pub async fn new() -> Result<AppState, Error> {
        let data_base_url = env::var("DATABASE_URL")?;
        let pool = MySqlPoolOptions::new()
            .max_connections(1)
            // .after_connect(|conn| Box::pin(async move {
            //     conn.execute("SET time_zone = '+08:00';").await?;
            //     Ok(())
            // }))
            .connect(&data_base_url)
            .await?;
        Ok(AppState { pool })
    }
}
