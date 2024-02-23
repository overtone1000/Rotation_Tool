use std::collections::HashMap;

use chrono::NaiveDate;

use crate::analysis::analysis_datum::{AnalysisDatum};

use super::coverage_and_work_day::CoverageAndWorkDay;

pub trait WorkCollector {
    fn collect_work(&self, workday: &CoverageAndWorkDay) -> AnalysisDatum;
    fn collect_work_bydate(
        &self,
        workday: &CoverageAndWorkDay,
    ) -> HashMap<NaiveDate, AnalysisDatum>;
}
