use chrono::{DateTime, Local};
use serde_json::Value as JsonValue;

use sqlx::{Executor, FromRow, MySqlPool};

use crate::{common::error::Error, modules::enums::AdType};

#[derive(Debug, Clone, FromRow)]
pub struct Slot {
    pub id: u64,
    pub created_at: DateTime<Local>,
    pub updated_at: DateTime<Local>,

    pub name: String,
    pub accept_ad_type: JsonValue,
    pub min_price_cpt: u32,
    pub min_price_cpm: u32,
    pub min_price_cpc: u32,
    pub width: u32,
    pub height: u32,
}

impl Slot {
    fn support_ad_type(&self, ad_type: AdType) -> bool {
        let ad_types = self.accept_ad_type.as_array().unwrap();
        for _ad_type in ad_types {
            if _ad_type.to_string() == ad_type.to_string() {
                return true;
            }
        }
        false
    }
}

pub async fn get_by_id(pool: &MySqlPool, id: u64) -> Result<Option<Slot>, Error> {
    let slot = sqlx::query_as("select * from dsp_slot where id = ?")
        .bind(id)
        .fetch_optional(pool)
        .await?;
    Ok(slot)
}
