use std::collections::HashMap;

use serde::{Serialize};

use crate::{analysis::analysis_datum::AnalysisDatum, coverage::{coverage_and_work_day::CoverageAndWorkDay, work_collector::WorkCollector}, serialization::weekday::SerializeableWeekday};



#[derive(Debug, PartialEq, Clone, Serialize)]
pub struct FractionalCoverageUnit {
    rotation: String,
    rotation_day: SerializeableWeekday,
    fraction: f64,
}

impl PartialOrd for FractionalCoverageUnit
{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.rotation_day.partial_cmp(&other.rotation_day) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        match self.rotation.partial_cmp(&other.rotation) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        self.fraction.partial_cmp(&other.fraction)
    }
}

impl Eq for FractionalCoverageUnit
{

}


impl Ord for FractionalCoverageUnit
{
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.partial_cmp(other)
        {
            Some(x) => x,
            None => std::cmp::Ordering::Equal,
        }
    }
}


impl FractionalCoverageUnit {
    pub fn create(
        rotation: String,
        weekday: chrono::Weekday,
        fraction: f64,
    ) -> FractionalCoverageUnit {
        FractionalCoverageUnit {
            rotation,
            rotation_day: SerializeableWeekday{day:weekday},
            fraction,
        }
    }

    pub fn get_rotation(&self) -> String {
        self.rotation.to_string()
    }
    pub fn get_day(&self) -> chrono::Weekday {
        self.rotation_day.day
    }
    pub fn get_fraction(&self) -> f64 {
        self.fraction
    }
}

impl WorkCollector for FractionalCoverageUnit {
    fn collect_work(&self, workday: &CoverageAndWorkDay) -> AnalysisDatum {
        let mut retval: AnalysisDatum = AnalysisDatum::default();

        for work in &workday.work {
            retval.add_workunit(work);
        }
        retval.scale(self.get_fraction());

        retval
    }

    fn collect_work_bydate(&self, workday: &CoverageAndWorkDay) -> HashMap<chrono::prelude::NaiveDate,AnalysisDatum> {
        let mut retval: HashMap<chrono::prelude::NaiveDate,AnalysisDatum> = HashMap::new();

        for work_unit in &workday.work {
            match retval.entry(work_unit.get_datetime().date())
            {
                std::collections::hash_map::Entry::Occupied(mut entry) => {
                    entry.get_mut().add_workunit(work_unit);
                }
                ,
                std::collections::hash_map::Entry::Vacant(empty) => {
                    let mut newdat=AnalysisDatum::default();
                    newdat.add_workunit(work_unit);
                    empty.insert(newdat);
                },
            };
        }

        for ad in retval.values_mut()
        {
            ad.scale(self.get_fraction());
        }

        retval
    }
}
