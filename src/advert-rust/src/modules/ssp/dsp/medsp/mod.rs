pub mod idea;
pub mod plan;
pub mod slot;
use async_std::task;
use std::{cell::RefCell, collections::HashMap};

use log::info;
use sqlx::MySqlPool;

use crate::{common::{app_state::AppState, config, error::Error}, modules::{ssp::{self, dsp_slot::DspSlot}, weight::{self, Weight}}};

use self::{
    idea::Idea,
    plan::{BidType, Plan, PlanConfigRule},
    slot::Slot,
};

use super::Dsp;

pub struct MeDsp<'a> {
    pool: &'a MySqlPool,
    config: &'a config::MeDspConfig,
    provider: String,
    slot: RefCell<Option<Slot>>,
}

impl<'a> Dsp for MeDsp<'a> {
    fn dsp(&self, req: &ssp::Request, slot: &DspSlot) -> Result<Vec<ssp::Ad>, Error> {
        task::block_on(self.do_dsp(req, slot))
    }
}

impl<'a> MeDsp<'a> {
    pub fn new(state: &'a AppState, provider: &str) -> MeDsp<'a> {
        MeDsp {
            pool: &state.pool,
            config: &state.config.dsp.me_dsp,
            provider: provider.to_owned(),
            slot: RefCell::new(None),
        }
    }

    async fn do_dsp(&self, req: &ssp::Request, slot: &DspSlot) -> Result<Vec<ssp::Ad>, Error> {
        let plans = plan::find_plans(self.pool).await?;
        if plans.is_empty() {
            return Ok(vec![]);
        }
        info!("plans len: {}", plans.len());

        let match_rule = MatchRule::new(req);
        let mut plan_ideas_vec = Vec::new();
        for plan in plans.iter() {
            let configs = plan::get_configs_by_plan_id(self.pool, plan.id).await?;
            for config in configs.iter() {
                let rules = plan::get_rules_by_config_id(self.pool, config.id).await?;
                for rule in rules.iter() {
                    if match_rule.r#match(rule) {
                        plan_ideas_vec.push(PlanIdeaIDs {
                            plan: plan.to_owned(),
                            idea_ids: config.idea_ids()?,
                        })
                    }
                }
            }
        }
        if plan_ideas_vec.is_empty() {
            return Ok(vec![]);
        }
        info!("plan_ideas_vec, len: {}", plan_ideas_vec.len());

        let (cpt, cpm, cpc) = self.group_by_bid_type(&plan_ideas_vec);

        let slot =
            slot::get_by_id(self.pool, slot.slot_id.unwrap())
                .await?
                .ok_or(Error::UnprocessableEntity(
                    422,
                    &format!("invalid slot_id: {}", slot.slot_id.unwrap()),
                ))?;
        *self.slot.borrow_mut() = Some(slot);

        let idea = self.do_dsp2("cpt", &cpt).await?;
        if idea.is_some() {
            return Err(Error::BadRequest(400, "400"));
        }
        let idea = self.do_dsp2("cpm", &cpm).await?;
        if idea.is_some() {
            return Err(Error::BadRequest(400, "400"));
        }
        let idea = self.do_dsp2("cpc", &cpc).await?;
        if idea.is_some() {
            return Err(Error::BadRequest(400, "400"));
        }
        Err(Error::BadRequest(400, "400"))
    }

    async fn do_dsp2(
        &self,
        group_name: &str,
        group: &[PlanIdeaIDs],
    ) -> Result<Option<Idea>, Error> {
        info!("group: {}", group_name);
        if group.is_empty() {
            info!("group is empty! return");
            return Ok(None);
        }

        let plan_ideaids = weight::random(group);
        info!("the final plan_ideaids: {:?}", plan_ideaids);

        let ideas = idea::find_by_ids(self.pool, &plan_ideaids.idea_ids).await?;
        info!("ideas len: {}", ideas.len());

        let ideas: Vec<Idea> = ideas
            .into_iter()
            .filter(|idea| {
                self.slot
                    .borrow()
                    .as_ref()
                    .unwrap()
                    .support_ad_type(idea.ad_type)
            })
            .collect();
        info!("after filter ideas len: {}", ideas.len());

        // todo:
        let idea = ideas[0].clone();
        info!("the final idea: {:?}", idea);

        Ok(Some(idea))
    }

    fn group_by_bid_type(
        &self,
        vec: &[PlanIdeaIDs],
    ) -> (Vec<PlanIdeaIDs>, Vec<PlanIdeaIDs>, Vec<PlanIdeaIDs>) {
        let mut cpt = Vec::new();
        let mut cpm = Vec::new();
        let mut cpc = Vec::new();

        for pii in vec {
            match pii.plan.bid_type {
                BidType::CPT => cpt.push(pii.clone()),
                BidType::CPM => cpm.push(pii.clone()),
                BidType::CPC => cpc.push(pii.clone()),
            }
        }
        cpt.sort_by(|a, b| a.plan.bid_price.cmp(&b.plan.bid_price).reverse());
        cpm.sort_by(|a, b| a.plan.bid_price.cmp(&b.plan.bid_price).reverse());
        cpc.sort_by(|a, b| a.plan.bid_price.cmp(&b.plan.bid_price).reverse());
        (cpt, cpm, cpc)
    }
}

#[derive(Debug, Clone)]
struct PlanIdeaIDs {
    plan: plan::Plan,
    idea_ids: Vec<u64>,
}

impl Weight for PlanIdeaIDs {
    fn weight(&self) -> u32 {
        self.plan.bid_price
    }
}

struct MatchRule<'a>(HashMap<&'a str, String>);

impl<'a> MatchRule<'a> {
    fn new(req: &ssp::Request) -> Self {
        let mut map = HashMap::new();
        map.insert("slot_id", req.slot.id.to_string());
        map.insert("os", req.device.os.to_string());

        //todo: other fields
        MatchRule(map)
    }

    fn r#match(&self, rule: &PlanConfigRule) -> bool {
        false
    }
}
