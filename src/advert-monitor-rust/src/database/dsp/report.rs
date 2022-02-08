use chrono::NaiveDate;
use itertools::Itertools;
use serde::Serialize;
use sqlx::{types::Decimal, FromRow};

use crate::{database::DSP_POOL, error::Error};

#[derive(Debug, Clone, FromRow, Serialize)]
pub struct Report {
    pub slot_id: i32,
    pub date: NaiveDate,
    pub req: Decimal,
    pub fil: Decimal,
    pub cli: Decimal,
    pub imp: Decimal,
}

#[derive(Clone, Copy)]
pub enum ReportCountField {
    Req,
    Fil,
    Cli,
    Imp,
}

impl Report {
    pub fn count(&self, field: ReportCountField) -> Decimal {
        match field {
            ReportCountField::Req => self.req,
            ReportCountField::Fil => self.fil,
            ReportCountField::Cli => self.cli,
            ReportCountField::Imp => self.imp,
        }
    }
}

pub async fn find_reports(
    slot_ids: &Vec<u32>,
    begin_date: &NaiveDate,
    end_date: &NaiveDate,
) -> Result<Vec<Report>, Error> {
    let sql = format!(
        r#"
select t.slot_id,
    t.date,
    sum(t.request) as req,
    sum(t.fill) as fil,
    sum(t.imp) as imp,
    sum(t.click) as cli
from report t
group by t.date,
    t.slot_id
having t.slot_id in ({})
    and t.date >= ?
    and t.date < ?
order by t.date desc
        "#,
        slot_ids.iter().map(|_| "?").join(","),
    );
    let mut query_as = sqlx::query_as(&sql);
    for slot_id in slot_ids {
        query_as = query_as.bind(slot_id);
    }
    query_as = query_as.bind(begin_date).bind(end_date);

    let reports = query_as.fetch_all(&*DSP_POOL).await?;
    Ok(reports)
}
