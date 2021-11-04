use chrono::{DateTime, Local};

use crate::{
    common::error::Error,
    modules::{enums::Status, weight::Weight},
};
use sqlx::{FromRow, MySqlPool};

#[derive(Debug, Clone, FromRow)]
pub struct Strategy {
    pub id: u64,
    pub created_at: DateTime<Local>,
    pub updated_at: DateTime<Local>,
    pub slot_id: u64,
    pub media_id: u64,

    pub name: String,
    pub status: Status,
}

#[derive(Debug, Clone, FromRow)]
pub struct StrategyConfig {
    pub id: u64,
    pub created_at: DateTime<Local>,
    pub updated_at: DateTime<Local>,
    pub strategy_id: u64,
}

#[derive(Debug, Clone, FromRow)]
pub struct TriggerRule {
    pub id: u64,
    pub created_at: DateTime<Local>,
    pub updated_at: DateTime<Local>,
    pub config_id: u64,

    pub key: String,
    pub value: String,
}

impl TriggerRule {
    pub fn contains(&self, value: &str) -> Result<bool, Error> {
        let values: Vec<String> = serde_json::from_str(&self.value)?;
        for _value in values.iter() {
            if _value == value {
                return Ok(true);
            }
        }
        Ok(false)
    }
}

#[derive(Debug, Clone, FromRow)]
pub struct TriggerTarget {
    pub id: u64,
    pub created_at: DateTime<Local>,
    pub updated_at: DateTime<Local>,
    pub config_id: u64,

    pub percent: u8,
    pub group1: String,
    pub group2: String,
}

impl TriggerTarget {
    fn id_group(group: &str) -> Result<Vec<u64>, Error> {
        Ok(serde_json::from_str(group)?)
    }

    pub fn group1(&self) -> Result<Vec<u64>, Error> {
        Self::id_group(&self.group1)
    }
    pub fn group2(&self) -> Result<Vec<u64>, Error> {
        Self::id_group(&self.group2)
    }
}

impl Weight for TriggerTarget {
    fn weight(&self) -> u32 {
        self.percent.into()
    }
}

#[derive(Debug, Clone, FromRow)]
pub struct BindDsp {
    pub id: u64,
    pub created_at: DateTime<Local>,
    pub updated_at: DateTime<Local>,

    pub percent: u8,
    pub dsp_provider: String,
    pub dsp_slot_id: u64,
}
impl Weight for BindDsp {
    fn weight(&self) -> u32 {
        self.percent.into()
    }
}

pub async fn get_by_slot_id_and_media_id(
    pool: &MySqlPool,
    slot_id: u64,
    media_id: u64,
) -> Result<Option<Strategy>, Error> {
    let strategy = sqlx::query_as!(Strategy, r#"
		select id, created_at as "created_at: _", updated_at as "updated_at: _", slot_id, media_id, name, status as "status: Status"
			from strategy where status = 'Valid' and slot_id = ? and media_id = ?"#, slot_id, media_id)
			.fetch_optional(pool)
			.await?;
    Ok(strategy)
}

pub async fn find_configs_by_strategy_id(
    pool: &MySqlPool,
    strategy_id: u64,
) -> Result<Vec<StrategyConfig>, Error> {
    let configs =
        sqlx::query_as::<_, StrategyConfig>("select * from strategy_config where strategy_id = ?")
            .bind(strategy_id)
            .fetch_all(pool)
            .await?;
    Ok(configs)
}

pub async fn find_rules_by_config_id(
    pool: &MySqlPool,
    config_id: u64,
) -> Result<Vec<TriggerRule>, Error> {
    let rules =
        sqlx::query_as::<_, TriggerRule>("select * from strategy_trigger_rule where config_id = ?")
            .bind(config_id)
            .fetch_all(pool)
            .await?;
    Ok(rules)
}

pub async fn find_trigger_targets_by_config_id(
    pool: &MySqlPool,
    config_id: u64,
) -> Result<Vec<TriggerTarget>, Error> {
    let targets = sqlx::query_as::<_, TriggerTarget>(
        "select * from strategy_trigger_target where config_id = ?",
    )
    .bind(config_id)
    .fetch_all(pool)
    .await?;
    Ok(targets)
}

pub async fn find_dsps_by_ids(pool: &MySqlPool, ids: Vec<u64>) -> Result<Vec<BindDsp>, Error> {
    let ids = ids
        .iter()
        .map(ToString::to_string)
        .collect::<Vec<String>>()
        .join(",");
    let dsps = sqlx::query_as("select * from strategy_bind_dsp where id in (?)")
        .bind(ids)
        .fetch_all(pool)
        .await?;
    Ok(dsps)
}
