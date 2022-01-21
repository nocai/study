pub mod dsp;

use serde::Serialize;

use crate::error::Error;
use std::any::Any;

#[derive(Debug, Serialize)]
pub struct Alarm {
    pub point: u32,
    pub details: serde_json::Value,
}

pub trait Monitor {
    fn name(&self) -> &str;
    fn open(&self) -> bool;
    fn obtain_data(&self) -> Result<Box<dyn Any>, Error>;
    fn process_data(&self, data: Box<dyn Any>) -> Result<Vec<Alarm>, Error>;
}

pub fn run(monitor: &Box<dyn Monitor>) -> Result<(), Error> {
    let data = monitor.obtain_data()?;

    let alarms = monitor.process_data(data)?;
    log::info!("alarms, len: {}", alarms.len());
    for alarm in alarms.iter() {
        log::info!("alarm: {:?}", alarm);
    }
    Ok(())
}
