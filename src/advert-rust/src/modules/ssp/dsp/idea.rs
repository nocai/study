use chrono::{DateTime, Local};
use sqlx::{Executor, MySqlPool};

use crate::modules::enums::AdType;

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct Idea {
    pub id: u64,
    pub created_at: DateTime<Local>,
    pub updated_at: DateTime<Local>,

    pub ad_type: AdType,
}

pub async fn get_by_id(pool: &MySqlPool, id: u64) -> Result<Option<Idea>, sqlx::Error> {
    sqlx::query_as("select * from dsp_idea where id = ?")
        .bind(id)
        .fetch_optional(pool)
        .await
}

pub async fn find_by_ids(pool: &MySqlPool, ids: &[u64]) -> Result<Vec<Idea>, sqlx::Error> {
    let ids = ids
        .iter()
        .map(ToString::to_string)
        .collect::<Vec<String>>()
        .join(",");
    sqlx::query_as("select * from dsp_idea where id in(?)")
        .bind(ids)
        .fetch_all(pool)
        .await
}
