use std::error::Error;

pub mod application;
pub mod config;
pub mod database;
pub mod error;
pub mod monitor;
pub mod model;

extern crate derive_more;
#[macro_use]
extern crate lazy_static;

fn main() -> Result<(), Box<dyn Error>> {
    dotenv::dotenv()?;
    env_logger::init();

    log::info!("config: {:?}", config::CONFIG.to_owned());
    let app = application::Application::new();
    Ok(app.run())
}
