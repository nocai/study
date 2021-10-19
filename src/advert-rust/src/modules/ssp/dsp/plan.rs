use chrono::{DateTime, Local};
use serde_json::Value as JsonValue;
use sqlx::{Executor, FromRow, MySqlPool, Type, Value};

use crate::common::error::Error;

#[derive(Debug, Clone, FromRow)]
pub struct Plan {
    pub id: u64,
    pub created_at: DateTime<Local>,
    pub updated_at: DateTime<Local>,
    // #[sqlx(default)]
    // pub configs: Vec<PlanConfig>,
    pub name: String,
    pub budget: u64,
    pub bid_type: BidType,
    pub bid_price: u32,

    pub weekdays: JsonValue,
    pub hours: JsonValue,

    pub begin_at: Option<DateTime<Local>>,
    pub end_at: Option<DateTime<Local>>,
}

#[derive(Debug, Clone, Type)]
pub enum BidType {
    CPT,
    CPC,
    CPM,
}

#[derive(Debug, Clone, FromRow)]
pub struct PlanConfig {
    pub id: u64,
    pub created_at: DateTime<Local>,
    pub updated_at: DateTime<Local>,

    pub plan_id: u64,
    pub idea_ids: JsonValue,
}

impl PlanConfig {
    pub fn idea_ids(&self) -> Result<Vec<u64>, Error> {
        let ids = self
            .idea_ids
            .as_array()
            .unwrap_or(&Vec::new())
            .iter()
            .filter(|v| v.is_u64())
            .map(|v| v.as_u64().unwrap())
            .collect();
        Ok(ids)
    }
}

#[derive(Debug, Clone, FromRow)]
pub struct PlanConfigRule {
    pub id: u64,
    pub created_at: DateTime<Local>,
    pub updated_at: DateTime<Local>,
    pub config_id: u64,

    pub key: String,
    pub value: JsonValue,
}

impl PlanConfigRule {
    fn contains(&self, value: &str) -> bool {
        self.value.as_array().unwrap().iter().any(|v| v == value)
    }
}

pub async fn get_by_id(pool: &MySqlPool, id: u64) -> Result<Option<Plan>, sqlx::Error> {
    sqlx::query_as("select * from dsp_plan where id = ?")
        .bind(id)
        .fetch_optional(pool)
        .await
}

pub async fn find_plans(pool: &MySqlPool) -> Result<Vec<Plan>, sqlx::Error> {
    let now = Local::now();
    sqlx::query_as!(Plan, r#"select id, created_at as "created_at: _", updated_at as "updated_at: _", name, budget, bid_type as "bid_type: BidType", bid_price, weekdays as "weekdays: JsonValue", hours as "hours: JsonValue", begin_at as "begin_at: _", end_at as "end_at: _" from dsp_plan
		where status = 'Valid' and begin_at <= ? and ? <= end_at"#, now, now)
		.fetch_all(pool)
		.await
}

pub async fn get_configs_by_plan_id(
    pool: &MySqlPool,
    plan_id: u64,
) -> Result<Vec<PlanConfig>, sqlx::Error> {
    sqlx::query_as("select * from dsp_plan_config wher plan_id = ?")
        .bind(plan_id)
        .fetch_all(pool)
        .await
}

pub async fn get_rules_by_config_id(
    pool: &MySqlPool,
    config_id: u64,
) -> Result<Vec<PlanConfigRule>, sqlx::Error> {
    sqlx::query_as("select * from dsp_plan_config_rule where config_id = ?")
        .bind(config_id)
        .fetch_all(pool)
        .await
}
