use itertools::Itertools;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

use crate::{database::SUPE_POOL, error::Error, model::OSType};

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Mng {
    pub id: i32,
    pub dsp_id: i32,
    #[sqlx(rename = "dspmedia_id")]
    pub media_id: String,
    #[sqlx(rename = "dspmedia_name")]
    pub media_name: String,
    #[sqlx(rename = "dspslot_id")]
    pub dsp_slot_id: String,
    #[sqlx(rename = "dspslot_name")]
    pub dsp_slot_name: String,
	#[sqlx(rename = "dspchannel_id")]
	pub channel_id: String,
    pub quota: i32,
    pub slot_type: i32,
    pub package: String,
    pub os_type: OSType,
    pub comment: String,
    pub width: i32,
    pub height: i32,
	pub cache_time: i32
}

pub async fn find_mngs(ids: Vec<i32>) -> Result<Vec<Mng>, Error> {
    let ids = ids.iter().join(",");
    let mngs = sqlx::query_as("select * from dsp_mng where id in (?)")
        .bind(ids)
        .fetch_all(&*SUPE_POOL)
        .await?;
    Ok(mngs)
}
