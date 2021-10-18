pub mod idea;
pub mod plan;
pub mod slot;

use std::collections::HashMap;

use sqlx::MySqlPool;

use crate::{common::error::Error, modules::ssp};

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
    Err(Error::BadRequest(400, "400"))
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
}
