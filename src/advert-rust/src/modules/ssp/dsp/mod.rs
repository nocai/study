pub mod idea;
pub mod plan;
pub mod slot;

use std::collections::HashMap;

use log::info;
use sqlx::MySqlPool;

use crate::{
    common::error::Error,
    modules::{
        ssp::{self},
        weight::{self, Weight},
    },
};

use self::{
    idea::Idea,
    plan::{BidType, Plan, PlanConfigRule},
    slot::Slot,
};

pub struct Request {
    pub req: ssp::Request,
    pub provider: String,
    pub slot_id: u64,
}

impl Request {
    fn new(req: ssp::Request, provider: String, slot_id: u64) -> Request {
        Request {
            req,
            provider,
            slot_id,
        }
    }
}

pub struct Response {}

pub async fn dsp(pool: &MySqlPool, req: &Request) -> Result<Option<Response>, Error> {
    let plans = plan::find_plans(pool).await?;
    if plans.is_empty() {
        return Ok(None);
    }
    info!("plans len: {}", plans.len());

    let match_rule = MatchRule::new(req);
    let mut plan_ideas_vec = Vec::new();
    for plan in plans.iter() {
        let configs = plan::get_configs_by_plan_id(pool, plan.id).await?;
        for config in configs.iter() {
            let rules = plan::get_rules_by_config_id(pool, config.id).await?;
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
        return Ok(None);
    }
    info!("plan_ideas_vec, len: {}", plan_ideas_vec.len());

    let (cpt, cpm, cpc) = group_by_bid_type(&plan_ideas_vec);

    let slot = slot::get_by_id(pool, req.slot_id)
        .await?
        .ok_or(Error::UnprocessableEntity(
            422,
            &format!("invalid slot_id: {}", req.slot_id),
        ))?;
    let idea = dsp2(pool, "cpt", &cpt, &slot).await?;
    if idea.is_some() {
        return Err(Error::BadRequest(400, "400"));
    }
    let idea = dsp2(pool, "cpm", &cpm, &slot).await?;
    if idea.is_some() {
        return Err(Error::BadRequest(400, "400"));
    }
    let idea = dsp2(pool, "cpc", &cpc, &slot).await?;
    if idea.is_some() {
        return Err(Error::BadRequest(400, "400"));
    }
    Err(Error::BadRequest(400, "400"))
}

async fn dsp2(
    pool: &MySqlPool,
    group_name: &str,
    group: &[PlanIdeaIDs],
    slot: &Slot,
) -> Result<Option<Idea>, Error> {
    info!("group: {}", group_name);
    if group.is_empty() {
        info!("group is empty! return");
        return Ok(None);
    }

    let plan_ideaids = weight::random(group);
    info!("the final plan_ideaids: {:?}", plan_ideaids);

    let ideas = idea::find_by_ids(pool, &plan_ideaids.idea_ids).await?;
    info!("ideas len: {}", ideas.len());

    let ideas: Vec<Idea> = ideas
        .into_iter()
        .filter(|idea| slot.support_ad_type(idea.ad_type))
        .collect();
    info!("after filter ideas len: {}", ideas.len());

    // todo:
    let idea = ideas[0].clone();
    info!("the final idea: {:?}", idea);

    Ok(Some(idea))
}

fn group_by_bid_type(
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
    fn new(req: &Request) -> Self {
        let mut map = HashMap::new();
        map.insert("slot_id", req.req.slot.id.to_string());
        map.insert("os", req.req.device.os_type.to_string());

        //todo: other fields
        MatchRule(map)
    }

    fn r#match(&self, rule: &PlanConfigRule) -> bool {
        false
    }
}
