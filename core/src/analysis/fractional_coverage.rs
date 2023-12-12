use super::{coverage_tree::{CoverageAndWorkDay, WorkCollector}, analysis_datum::AnalysisDatum};

#[derive(Debug, PartialEq, Clone)]
pub struct FractionalCoverageUnit {
    rotation: String,
    rotation_day: chrono::Weekday,
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
            rotation_day: weekday,
            fraction,
        }
    }

    pub fn get_rotation(&self) -> String {
        self.rotation.to_string()
    }
    pub fn get_day(&self) -> chrono::Weekday {
        self.rotation_day
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
