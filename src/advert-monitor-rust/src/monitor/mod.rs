pub mod dsp;
pub mod ssp;

use async_std::task::block_on;
use job_scheduler::{Job, JobScheduler};
use serde::Serialize;

use crate::{config, error::Error};
use std::{
    any::Any,
    thread::{sleep, spawn},
    time::Duration,
};

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

pub fn run(monitors: &Vec<Box<dyn Monitor>>) {
    let mut sched = JobScheduler::new();
    sched.add(Job::new(
        config::CONFIG.monitor.cron.parse().unwrap(),
        || {
            for monitor in monitors.iter() {
                if let Err(err) = block_on(do_run(&monitor)) {
                    log::error!("run monitor: {:?}, error: {:?}", monitor.name(), err);
                }
            }
        },
    ));
    loop {
        sched.tick();
        sleep(Duration::from_millis(500));
    }
}

pub async fn do_run(monitor: &Box<dyn Monitor>) -> Result<(), Error> {
    log::info!("monitor: {:?} is runing.", monitor.name());
    if !monitor.open() {
        log::info!("monitor: {:?} is close.", monitor.name());
        return Ok(());
    }
    let data = monitor.obtain_data()?;

    let alarms = monitor.process_data(data)?;
    log::info!("alarms, len: {}", alarms.len());

    for alarm in alarms.iter() {
        log::info!("alarm: {:?}", alarm);
    }

    log::info!("monitor: {:?} is finished.", monitor.name());
    Ok(())
}
