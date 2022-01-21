use std::{thread::sleep, time::Duration};

use job_scheduler::{Job, JobScheduler};

use crate::{
    config,
    monitor::{self, dsp::DspMonitor, Monitor},
};

pub struct Application {
    monitors: Vec<Box<dyn Monitor>>,
}

impl Application {
    pub fn new() -> Application {
        let monitors: Vec<Box<dyn Monitor>> = vec![Box::new(DspMonitor {})];
        // monitors.push(Box::new(DspMonitor {}));
        Application { monitors }
    }

    pub fn run(&self) {
        let mut sched = JobScheduler::new();
        sched.add(Job::new(
            config::CONFIG.monitor.cron.parse().unwrap(),
            || {
                for _monitor in self.monitors.iter() {
                    log::info!("monitor: {:?} is runing.", _monitor.name());
                    if !_monitor.open() {
                        log::info!("monitor: {:?} is close.", _monitor.name());
                        continue;
                    }
                    if let Err(err) = monitor::run(_monitor) {
                        log::error!("run monitor: {:?}, error: {:?}", _monitor.name(), err);
                    }
                    log::info!("monitor: {:?} is finished.", _monitor.name());
                }
            },
        ));
        loop {
            sched.tick();
            sleep(Duration::from_millis(500));
        }
    }
}
