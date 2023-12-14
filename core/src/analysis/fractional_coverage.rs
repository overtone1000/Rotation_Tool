use serde::{Serialize, ser::SerializeStruct};

use super::{coverage_tree::{CoverageAndWorkDay, WorkCollector}, analysis_datum::AnalysisDatum, temporal_coverage::weekday_for_javascript};

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct SerializeableWeekday
{
    pub day:chrono::Weekday
}

impl SerializeableWeekday
{
    pub fn new(weekday:chrono::Weekday)->SerializeableWeekday{
        SerializeableWeekday{day:weekday}
    }
}

impl Serialize for SerializeableWeekday
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer {
        serializer.serialize_u32(weekday_for_javascript(&self.day))
    }
}

#[derive(Debug, PartialEq, Clone, Serialize)]
pub struct FractionalCoverageUnit {
    rotation: String,
    rotation_day: SerializeableWeekday,
    fraction: f64,
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
}
