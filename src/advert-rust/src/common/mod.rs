use crate::common::error::Error;

pub mod app_state;
pub mod error;

pub fn init() {
    dotenv::dotenv().ok();
    env_logger::init();
}
