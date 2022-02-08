use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use sqlx::{
    error::BoxDynError,
    mysql::{MySqlTypeInfo, MySqlValueRef},
    Decode, FromRow, MySql, Type,
};

use crate::{
    database::{supe::mng, SUPE_POOL},
    error::{Error, InternalServer},
    model::{Status, Version},
    monitor::ssp::KV,
};

use super::mng::Mng;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Strategy {
    pub id: i32,
    pub status: Status,
    pub config: StrategyConfigs,
    pub comment: String,
    #[sqlx(rename = "create_time")]
    pub created_at: DateTime<Local>,
    #[sqlx(rename = "modify_time")]
    pub updated_at: DateTime<Local>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StrategyConfigs {
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

const STRATEGY_CONFIG_DATA_RULE_KEY_APPV: &str = "appv";

impl StrategyConfigRules {
    pub fn app_version(&self) -> Result<Version, Error> {
        for rule in self.0.iter() {
            if let Ok(app_versions) = Self::app_versions(rule) {
                if let Some(app_version) = app_versions.get(0) {
                    return Ok(app_version.clone());
                }
            }
        }
        InternalServer("no app rule.")
    }

    pub fn app_versions(rule: &KV) -> Result<Vec<Version>, Error> {
        if rule.key != STRATEGY_CONFIG_DATA_RULE_KEY_APPV {
            return InternalServer(&format!("not app. rule: {:?}", rule));
        }

        // 9.35,9.36,9.37,9.38,9.39,9.40,9.41,9.42,9.43,9.44,9.45
        let mut versions = Vec::new();
        for value in rule.value.split(",").collect::<Vec<&str>>() {
            if let Ok(version) = value.try_into() {
                versions.push(version)
            }
        }

        Ok(versions)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StrategyConfigDistribution {
    pub percent: i32,
    #[serde(rename = "firstDspInfo")]
    pub first_dsp_infos: StrategyConfigDistributionDspInfos,
    #[serde(rename = "secondDspInfo")]
    pub second_dsp_infos: StrategyConfigDistributionDspInfos,
}

impl StrategyConfigDistribution {
    pub(crate) fn find_dsp_infos(&self, dsp_id: i32) -> StrategyConfigDistributionDspInfos {
        let mut first_dsp_infos = Self::do_find_dsp_infos(&self.first_dsp_infos, dsp_id);
        let mut second_dsp_infos = Self::do_find_dsp_infos(&self.second_dsp_infos, dsp_id);
        first_dsp_infos.0.append(&mut second_dsp_infos.0);
        first_dsp_infos
    }

    pub fn do_find_dsp_infos(
        dsp_infos: &StrategyConfigDistributionDspInfos,
        dsp_id: i32,
    ) -> StrategyConfigDistributionDspInfos {
        let mut vec = Vec::with_capacity(dsp_infos.0.capacity());
        for dsp_info in dsp_infos.0.iter() {
            if dsp_info.dsp_id == dsp_id {
                vec.push(dsp_info.clone());
            }
        }
        StrategyConfigDistributionDspInfos(vec)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StrategyConfigDistributionDspInfos(pub Vec<StrategyConfigDistributionDspInfo>);
impl StrategyConfigDistributionDspInfos {
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

impl Decode<'_, MySql> for StrategyConfigs {
    fn decode(value: MySqlValueRef<'_>) -> Result<Self, BoxDynError> {
        let value = <&str as Decode<MySql>>::decode(value)?;
        Ok(serde_json::from_str(value)?)
    }
}

impl Type<MySql> for StrategyConfigs {
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

    return InternalServer(&format!(
        "invalid slot_id: {} or media_id: {}",
        slot_id, media_id
    ));
}

#[cfg(test)]
mod tests {
    use crate::{model::Version, monitor::ssp::KV};

    use super::{StrategyConfigRules, STRATEGY_CONFIG_DATA_RULE_KEY_APPV};

    #[test]
    fn strategy_config_rule_app_versions() {
        let kv = KV {
            key: STRATEGY_CONFIG_DATA_RULE_KEY_APPV.to_owned(),
            value: "9.35,9.36,9.37,9.38,9.39,9.40,9.41,9.42,9.43,9.44,9.45".to_owned(),
        };
        let app_versions = StrategyConfigRules::app_versions(&kv);
        log::info!("{:?}", app_versions);
        assert_eq!(
            app_versions.unwrap()[0],
            Version {
                major: 9,
                minor: 35,
                micro: 0
            }
        )
    }
}
