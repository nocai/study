use chrono::{DateTime, Duration, Local, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::{database::HasValueRef, mysql::MySqlTypeInfo, Database, Decode, FromRow, MySql, Type};

use crate::{
    database::DSP_POOL,
    error::Error,
    model::{BidType, Status},
};

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Plan {
    pub id: i32,
    #[sqlx(rename = "status")]
    pub status: Status,
    #[sqlx(rename = "create_time")]
    pub created_at: DateTime<Local>,
    #[sqlx(rename = "modify_time")]
    pub updated_at: DateTime<Local>,

    pub name: String,
    #[sqlx(rename = "user")]
    pub user_id: i32,
    pub budget: i32,
    pub bid_type: BidType,
    pub bid_price: i32,
    pub weekday: String,
    pub hour: String,
    pub adconfig: AdConfigVec,
    pub begin_time: Option<DateTime<Local>>,
    pub end_time: Option<DateTime<Local>>,
    pub closed_time: Option<DateTime<Local>>,
    pub closed_reason: Option<String>,
}

impl Plan {
    pub fn time_overlap(&self, other: &Plan, allow_overlap_max: Duration) -> bool {
        let beginend_time_overlap = self.beginend_time_overlap(other, allow_overlap_max);
        let weekday_overlap = self.weekday_overlap(other);
        let hour_overlap = self.hour_overlap(other);
        beginend_time_overlap && weekday_overlap && hour_overlap
    }

    pub fn beginend_time_overlap(&self, other: &Plan, allow_overlap_max: Duration) -> bool {
        let now = Local::now();

        let mut begin_time = self.begin_time.unwrap_or(now);
        let other_begin_time = other.begin_time.unwrap_or(now);
        if begin_time < other_begin_time {
            begin_time = other_begin_time;
        }

        let mut end_time = self.end_time.unwrap_or(now);
        let other_end_time = other.end_time.unwrap_or(now);
        if end_time > other_end_time {
            end_time = other_end_time
        }

        (end_time - begin_time) > allow_overlap_max
    }

    pub fn weekday_overlap(&self, other: &Plan) -> bool {
        let weekdays: Vec<&str> = self.weekday.split(",").collect();
        let other_weekdays: Vec<&str> = other.weekday.split(",").collect();
        for weekday in weekdays {
            for other_weekday in &other_weekdays {
                if weekday == *other_weekday {
                    return true;
                }
            }
        }
        false
    }

    pub fn hour_overlap(&self, other: &Plan) -> bool {
        let hours: Vec<&str> = self.hour.split(",").collect();
        let other_hours: Vec<&str> = other.hour.split(",").collect();
        for hour in hours {
            for other_hour in &other_hours {
                if hour == *other_hour {
                    return true;
                }
            }
        }
        false
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdConfigVec(pub Vec<AdConfig>);

const RULE_KEY_SLOT_ID: &str = "slotid";
const RULE_KEY_TAG: &str = "tag";

impl AdConfigVec {
    pub fn gen_rules(&self) -> Vec<String> {
        let mut rules = Vec::new();
        for ac in self.0.iter() {
            rules.append(&mut ac.gen_rules());
        }
        return rules;
    }
    pub fn has_slot(rules: &Vec<String>) -> bool {
        for rule in rules {
            if rule.contains(RULE_KEY_SLOT_ID) {
                return true;
            }
        }
        false
    }
}

impl<'r, DB: Database> Decode<'r, DB> for AdConfigVec
where
    &'r str: Decode<'r, DB>,
{
    fn decode(
        value: <DB as HasValueRef<'r>>::ValueRef,
    ) -> Result<AdConfigVec, Box<dyn std::error::Error + 'static + Send + Sync>> {
        let value = <&str as Decode<DB>>::decode(value)?;
        Ok(serde_json::from_str(value)?)
    }
}

impl Type<MySql> for AdConfigVec {
    fn type_info() -> <MySql as Database>::TypeInfo {
        <str as Type<MySql>>::type_info()
    }
    fn compatible(ty: &MySqlTypeInfo) -> bool {
        <str as Type<MySql>>::compatible(ty)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
pub struct AdConfig {
    pub ideas: Vec<i32>,
    pub rules: Vec<AdConfigRule>,
}

impl AdConfig {
    fn gen_rules(&self) -> Vec<String> {
        let mut rules = Vec::new();
        for rule in self.rules.iter() {
            let mut tmp = rule.gen_rules();
            if rules.is_empty() {
                rules.append(&mut tmp);
            } else {
                rules = Self::gen_rules_new(&rules, &tmp);
            }
        }
        rules
    }

    fn gen_rules_new(rules: &Vec<String>, rules_new: &Vec<String>) -> Vec<String> {
        let mut vec = Vec::with_capacity(rules.capacity());
        for rule_new in rules_new {
            for rule_old in rules {
                vec.push(format!("{}-{}", rule_old, rule_new))
            }
        }
        vec
    }

    pub fn tag_rules(&self) -> Vec<AdConfigRule> {
        let mut tag_rules = Vec::new();
        for rule in self.rules.iter() {
            if rule.key == RULE_KEY_TAG {
                tag_rules.push(rule.clone())
            }
        }
        tag_rules
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdConfigRule {
    pub key: String,
    pub values: Vec<serde_json::Value>,
}

impl AdConfigRule {
    fn gen_rules(&self) -> Vec<String> {
        let mut rules = Vec::new();
        for v in self.values.iter() {
            rules.push(format!("{}:{}", &self.key, &v))
        }
        return rules;
    }

    pub fn contains(&self, key: &str, value: &str) -> bool {
        if self.key == RULE_KEY_TAG {
            return true;
        }
        if self.key != key {
            return false;
        }

        for v in self.values.iter() {
            if v.to_string() == value {
                return true;
            }
        }
        false
    }

    pub fn tags(&self) -> Vec<Value> {
        if self.key != RULE_KEY_TAG {
            return vec![];
        }

        self.values.clone()
    }
}

pub async fn find_valid_plans() -> Result<Vec<Plan>, Error> {
    let plans = sqlx::query_as("select * from plan where status = 0 and ? < end_time")
        .bind(Utc::now())
        .fetch_all(&*DSP_POOL)
        .await?;
    Ok(plans)
}
