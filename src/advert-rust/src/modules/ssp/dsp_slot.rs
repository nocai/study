use chrono::{DateTime, Local};

use crate::modules::enums::OS;

use serde::{Deserialize, Serialize};
use sqlx::{FromRow, MySqlPool};

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct DspSlot {
    pub id: u64,
    pub created_at: DateTime<Local>,
    pub updated_at: DateTime<Local>,

    pub media_id: Option<u64>,
    pub slot_id: Option<u64>,
    pub slot_type: Option<String>,

    pub os: Option<OS>,
    pub package: Option<String>,
    pub width: u32,
    pub height: u32,
}

pub async fn get_by_id(pool: &MySqlPool, id: u64) -> Result<Option<DspSlot>, sqlx::Error> {
    let v_dsp_slot = sqlx::query_as!(DspSlot, r#"
		select id, created_at as "created_at: _", updated_at as "updated_at: _", media_id, slot_id, slot_type, os as "os: OS", package, width, height
			from v_dsp_slot where id = ?"#, id)
			.fetch_optional(pool)
			.await?;
    Ok(v_dsp_slot)
}
