use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use sqlx::{
    error::BoxDynError,
    mysql::{MySqlTypeInfo, MySqlValueRef},
    Decode, FromRow, MySql, Type,
};

use crate::{
    database::{supe::mng, SUPE_POOL},
    error::{self, Error, InternalServer},
    model::{Status, Version},
    monitor::ssp::KV,
};

use super::mng::Mng;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Strategy {
    pub id: i32,
    pub status: Status,
    pub config: StrategyConfigVec,
    pub comment: String,
    #[sqlx(rename = "create_time")]
    pub created_at: DateTime<Local>,
    #[sqlx(rename = "modify_time")]
    pub updated_at: DateTime<Local>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StrategyConfigVec {
    pub data: Vec<StrategyConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StrategyConfig {
    #[serde(rename = "result")]
    pub distributions: Vec<StrategyConfigDistribution>,
    pub rules: StrategyConfigRules,
}

impl StrategyConfig {
    pub fn app_version(&self) -> Result<Version, Error> {
        if let Ok(app_version) = self.rules.app_version() {
            return Ok(app_version);
        }
        return InternalServer(&format!("no app rule. rules: {:?}", self.rules));
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StrategyConfigRules(Vec<KV>);

const STRATEGYCONFIGDATARULEKEY_APPV: &str = "appv";

impl StrategyConfigRules {
    pub fn app_version(&self) -> Result<Version, Error> {
        for rule in self.0.iter() {
            if let Ok(app_version) = Self::do_app_version(rule) {
                return Ok(app_version);
            }
        }
        error::InternalServer("no app rule.")
    }

    pub fn do_app_version(rule: &KV) -> Result<Version, Error> {
        if rule.key != STRATEGYCONFIGDATARULEKEY_APPV {
            return error::InternalServer(&format!("not app. rule: {:?}", rule));
        }

        // Fixme: 这里怎么写比较好？
        if let Some(value) = rule.value.split(",").next() {
            let mut value = value.split(".");
            if let Some(major) = value.next() {
                if let Ok(major) = major.parse::<u32>() {
                    let mut app_version: Version = Version::default();
                    app_version.major = major;

                    if let Some(minor) = value.next() {
                        if let Ok(minor) = minor.parse::<u32>() {
                            app_version.minor = minor;
                        }
                    }

                    if let Some(micro) = value.next() {
                        if let Ok(micro) = micro.parse::<u32>() {
                            app_version.micro = micro;
                        }
                    }
                    return Ok(app_version);
                }
            }
        }
        return InternalServer(&format!("not app. rule: {:?}", rule));
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StrategyConfigDistribution {
    pub percent: i32,
    #[serde(rename = "firstDspInfo")]
    pub first_dsp_infos: StrategyConfigDistributionDspInfoVec,
    #[serde(rename = "secondDspInfo")]
    pub second_dsp_infos: StrategyConfigDistributionDspInfoVec,
}

impl StrategyConfigDistribution {
    pub(crate) fn find_dsp_infos(&self, dsp_id: i32) -> StrategyConfigDistributionDspInfoVec {
        let mut first_dsp_infos = Self::do_find_dsp_infos(&self.first_dsp_infos, dsp_id);
        let mut second_dsp_infos = Self::do_find_dsp_infos(&self.second_dsp_infos, dsp_id);
        first_dsp_infos.0.append(&mut second_dsp_infos.0);
        first_dsp_infos
    }

    pub fn do_find_dsp_infos(
        dsp_infos: &StrategyConfigDistributionDspInfoVec,
        dsp_id: i32,
    ) -> StrategyConfigDistributionDspInfoVec {
        let mut vec = Vec::with_capacity(dsp_infos.0.capacity());
        for dsp_info in dsp_infos.0.iter() {
            if dsp_info.dsp_id == dsp_id {
                vec.push(dsp_info.clone());
            }
        }
        StrategyConfigDistributionDspInfoVec(vec)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StrategyConfigDistributionDspInfoVec(pub Vec<StrategyConfigDistributionDspInfo>);
impl StrategyConfigDistributionDspInfoVec {
    pub async fn mngs(&self) -> Result<Vec<Mng>, Error> {
        let mng_ids = self.0.iter().map(|it| it.dsp_mng_id).collect();
        mng::find_mngs(mng_ids).await
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StrategyConfigDistributionDspInfo {
    #[serde(rename = "dspId")]
    pub dsp_id: i32,
    #[serde(rename = "dspCpm")]
    pub dsp_cpm: i32,
    #[serde(rename = "dspPercent")]
    pub dsp_percent: i32,
    #[serde(rename = "dspMngId")]
    pub dsp_mng_id: i32,
    pub config: String,
}

impl Decode<'_, MySql> for StrategyConfigVec {
    fn decode(value: MySqlValueRef<'_>) -> Result<Self, BoxDynError> {
        let value = <&str as Decode<MySql>>::decode(value)?;
        Ok(serde_json::from_str(value)?)
    }
}

impl Type<MySql> for StrategyConfigVec {
    fn type_info() -> MySqlTypeInfo {
        <str as Type<MySql>>::type_info()
    }

    fn compatible(ty: &MySqlTypeInfo) -> bool {
        <str as Type<MySql>>::compatible(ty)
    }
}

pub async fn get_by_id(id: i32) -> Result<Option<Strategy>, Error> {
    let strategy = sqlx::query_as("select * from dsp_strategy where id = ?")
        .bind(id)
        .fetch_optional(&*SUPE_POOL)
        .await?;
    Ok(strategy)
}

pub async fn get_by_slotid_or_mediaid(slot_id: i32, media_id: i32) -> Result<Strategy, Error> {
    if let Some(strategy) = get_by_id(slot_id).await? {
        return Ok(strategy);
    }

    if let Some(strategy) = get_by_id(media_id).await? {
        return Ok(strategy);
    }

    return error::InternalServer(&format!(
        "invalid slot_id: {} or media_id: {}",
        slot_id, media_id
    ));
}
