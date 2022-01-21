use std::collections::HashMap;

use async_std::task::block_on;
use chrono::Duration;
use serde_json::json;

use crate::{
    config,
    database::plan::{self, AdConfigVec, Plan},
    monitor::Alarm,
};

use super::Monitor;

pub struct DspMonitor {}

impl Monitor for DspMonitor {
    fn name(&self) -> &str {
        "DSP"
    }

    fn open(&self) -> bool {
        config::CONFIG.monitor.dsp.open
    }

    fn obtain_data(&self) -> Result<Box<dyn std::any::Any>, crate::error::Error> {
        let plans = block_on(plan::find_valid_plans())?;
        log::info!("plans: {:?}", &plans);
        Ok(Box::new(plans))
    }

    fn process_data(
        &self,
        data: Box<dyn std::any::Any>,
    ) -> Result<Vec<super::Alarm>, crate::error::Error> {
        let plans = data.downcast::<Vec<Plan>>().unwrap();

        let config = &config::CONFIG.monitor.dsp;
        let allow_over_max = Duration::seconds(config.allow_overlap_max);

        let mut alarms = Vec::new();
        let mut alarm = |rule: &str, plans: &Vec<&Plan>| {
            log::info!("rule: {:?}, plans({}): {:?}", rule, plans.len(), plans);
            for i in 0..plans.len() {
                for j in (i + 1)..plans.len() {
                    if plans[i].time_overlap(&plans[j], allow_over_max) {
                        alarms.push(Alarm {
                            point: 4,
                            details: json!({
                                "rule": rule,
                                "plans": vec![plans[i].clone(), plans[j].clone()],
                            }),
                        })
                    }
                }
            }
        };

        let group = group(&plans);
        for (rule, plans) in group {
            alarm(&rule, &plans);
        }
        Ok(alarms)
    }
}

fn group(plans: &Vec<Plan>) -> HashMap<String, Vec<&Plan>> {
    let mut groups: HashMap<String, Vec<&Plan>> = HashMap::new();
    for plan in plans.iter() {
        let rules = plan.adconfig.gen_rules();
        if !AdConfigVec::has_slot(&rules) {
            continue;
        }

        for rule in rules {
            if let Some(plans) = groups.get_mut(&rule) {
                plans.push(plan)
            } else {
                groups.insert(rule, vec![plan]);
            }
        }
    }
    groups
}
