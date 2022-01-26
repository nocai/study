use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

use crate::{
    database::SUPE_POOL,
    error::Error,
    model::{MediaType, OSType},
};

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Media {
    pub id: i32,
    pub name: String,
    #[sqlx(rename = "user")]
    pub user_id: i32,
    #[sqlx(rename = "type")]
    pub media_type: MediaType,
    pub os_type: OSType,
    pub source_name: String,
    pub source_url: String,
    #[sqlx(rename = "create_time")]
    pub created_at: DateTime<Local>,
    #[sqlx(rename = "modify_time")]
    pub updated_at: DateTime<Local>,
}

pub async fn get_by_id(id: i32) -> Result<Media, Error> {
    let media = sqlx::query_as("select * from media where id = ?")
        .bind(id)
        .fetch_one(&*SUPE_POOL)
        .await?;
    Ok(media)
}
