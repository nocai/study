use std::collections::HashMap;

use async_std::task::block_on;
use chrono::{Date, Datelike, Duration, Local, NaiveDate, Weekday};
use serde_json::json;
use sqlx::types::Decimal;

use crate::{
    config::{self, ReportMonitorConfig},
    database::dsp::report::{self, Report, ReportCountField},
    error::Error,
};

use super::{Alarm, Monitor};

pub struct ReportMonitor {}

impl Monitor for ReportMonitor {
    fn name(&self) -> &str {
        "Report"
    }

    fn open(&self) -> bool {
        config::CONFIG.monitor.report.open
    }

    fn obtain_data(&self) -> Result<Box<dyn std::any::Any>, crate::error::Error> {
        let config = &config::CONFIG.monitor.report;

        let today = Local::today().naive_local();
        let begin_date = today - Duration::days(config.window_days.into());
        log::info!(
            "slot_ids: {:?}, begin_date: {:?}, end_date: {:?}",
            config.slot_ids,
            begin_date,
            today
        );

        let reports = block_on(report::find_reports(&config.slot_ids, &begin_date, &today))?;
        log::info!("reports({}): {:?}", reports.len(), reports);

        Ok(Box::new(reports))
    }

    fn process_data(
        &self,
        data: Box<dyn std::any::Any>,
    ) -> Result<Vec<super::Alarm>, crate::error::Error> {
        let reports = data.downcast::<Vec<Report>>().unwrap();
        block_on(process_reports(*reports))
    }
}

async fn process_reports(reports: Vec<Report>) -> Result<Vec<Alarm>, Error> {
    let config = &config::CONFIG.monitor.report;

    // filters
    let reports = reports
        .into_iter()
        // 排队周末
        .filter(|it| {
            let is_weekend = it.date.weekday() == Weekday::Sat || it.date.weekday() == Weekday::Sun;
            if is_weekend {
                log::info!("filter by weekend. weekday: {:?}", it.date.weekday());
            }
            !is_weekend
        })
        // 排除该广告位某天是否投放, 设置当天统计的填充数是否大于最低限制, 默认10000
        .filter(|it| {
            let satisfy = it.fil < config.fill_count_min.into();
            if satisfy {
                log::info!(
                    "filter by fill_count_min: {}. fil: {}",
                    config.fill_count_min,
                    it.fil
                );
            }
            !satisfy
        })
        .collect::<Vec<Report>>();
    log::info!("after filter, reports({}): {:?}", reports.len(), reports);

    let mut group = group(reports).await;
    log::info!("group({}): {:?}", group.len(), group);

    let mut alarms = Vec::new();
    for (slot_id, reports) in group.iter_mut() {
        if reports.len() < config.exclude_days as usize {
            log::info!(
                "排除后剩余天数: {} < exclude_days: {}, 此段时间窗无效",
                reports.len(),
                config.exclude_days
            );
            continue;
        }

        reports.sort_by(|a, b| a.date.cmp(&b.date));
        log::info!(
            "slot_id: {}, reports({}): {:?}",
            slot_id,
            reports.len(),
            reports
        );

        let map = calc(config, reports.to_owned(), ReportCountField::Req).await;
        log::info!("calc req result: {:?}", map);
        for (count, reports) in map {
            if count > config.countdown_days {
                alarms.push(build_alarm(5, reports));
            }
        }

        let map = calc(config, reports.to_owned(), ReportCountField::Fil).await;
        log::info!("calc fil result: {:?}", map);
        for (count, reports) in map {
            if count > config.countdown_days {
                alarms.push(build_alarm(6, reports));
            }
        }

        let map = calc(config, reports.to_owned(), ReportCountField::Cli).await;
        log::info!("calc cli result: {:?}", map);
        for (count, reports) in map {
            if count > config.countdown_days {
                alarms.push(build_alarm(7, reports));
            }
        }

        let map = calc(config, reports.to_owned(), ReportCountField::Imp).await;
        log::info!("calc imp result: {:?}", map);
        for (count, reports) in map {
            if count > config.countdown_days {
                alarms.push(build_alarm(8, reports));
            }
        }
    }
    Ok(alarms)
}

fn build_alarm(point: u32, reports: Vec<Report>) -> Alarm {
    Alarm {
        point,
        details: json!({
            "report": reports,
        }),
    }
}
struct CalcTmp {
    pub count: u32,
    pub reports: Vec<Report>,
}

impl CalcTmp {
    fn new() -> CalcTmp {
        CalcTmp {
            count: 0,
            reports: Vec::new(),
        }
    }
}

// Fixme
async fn calc(
    config: &ReportMonitorConfig,
    reports: Vec<Report>,
    field: ReportCountField,
) -> HashMap<u32, Vec<Report>> {
    let mut map = HashMap::new();

    let mut tmp = CalcTmp::new();
    for idx in 0..reports.len() - 1 {
        let cur = &reports[idx];
        let next = &reports[idx + 1];

        tmp.reports.push(cur.clone());
        if count_down(&cur, &next, field) {
            log::info!("count down, {} -> {}", cur.count(field), next.count(field));
            tmp.count += 1;
            // next是最后一个
            if idx + 1 == reports.len() {
                tmp.reports.push(next.clone());
                map.insert(tmp.count, tmp.reports);
                return map;
            }
        } else if count_up(&cur, &next, field) {
            log::info!("count up, {} -> {}", cur.count(field), next.count(field));
            map.insert(tmp.count, tmp.reports);
            tmp = CalcTmp::new();
        }
    }
    map
}

fn count_down(cur: &Report, next: &Report, field: ReportCountField) -> bool {
    cur.count(field) > next.count(field)
}

fn count_up(cur: &Report, next: &Report, field: ReportCountField) -> bool {
    cur.count(field) < next.count(field)
}

async fn group(reports: Vec<Report>) -> HashMap<i32, Vec<Report>> {
    let mut map: HashMap<i32, Vec<Report>> = HashMap::new();
    for report in reports {
        if let Some(reports) = map.get_mut(&report.slot_id) {
            reports.push(report);
        } else {
            map.insert(report.slot_id, vec![report]);
        }
    }
    map
}
