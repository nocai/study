use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

use crate::{database::SUPE_POOL, error::Error, model::Status};

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Slot {
    pub id: i32,
    pub name: String,
    #[sqlx(rename = "media")]
    pub media_id: i32,
    pub status: Status,
    pub config: String,
    #[sqlx(rename = "create_time")]
    pub created_at: DateTime<Local>,
    #[sqlx(rename = "modify_time")]
    pub updated_at: DateTime<Local>,
    pub quota: i32,
    pub tpl_id: i32,
}

#[derive(Deserialize)]
pub struct SlotConfig {
    #[serde(rename = "refreshType")]
    pub refresh_type: i32,
    pub round: i32,
}

pub async fn get_by_id(id: i32) -> Result<Slot, Error> {
    let slot = sqlx::query_as("select * from adslot where id = ?")
        .bind(id)
        .fetch_one(&*SUPE_POOL)
        .await?;
    Ok(slot)
}
