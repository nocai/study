use crate::monitor::{self, dsp::DspMonitor, report::ReportMonitor, ssp::SspMonitor, Monitor};

pub struct Application {
    monitors: Vec<Box<dyn Monitor>>,
}

impl Application {
    pub fn new() -> Application {
        let monitors: Vec<Box<dyn Monitor>> = vec![
            Box::new(DspMonitor {}),
            Box::new(SspMonitor {}),
            Box::new(ReportMonitor {}),
        ];
        Application { monitors }
    }

    pub fn run(&self) {
        monitor::run(&self.monitors);
    }
}
