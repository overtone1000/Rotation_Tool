use std::{
    collections::{hash_map::Entry, HashMap},
    error::Error,
};

use crate::{
    analysis::analysis_datum::AnalysisDatum,
    coverage::{
        coordinate::CoverageCoordinates, coverage_and_work_day::CoverageAndWorkDay,
        units::Coverage, work_collector::WorkCollector, work_coverage_map::CoverageMap,
    },
    globals::ALL_DAYS,
};

pub fn details(
    coverage_map: &mut CoverageMap,
    analyzed_weekday: chrono::Weekday,
    analyzed_rotation: &str,
) -> Result<AnalysisDatum, Box<dyn Error>> {
    let mut aggregate: AnalysisDatum = AnalysisDatum::default();

    let mut addfunc = |rotation: String, weekday: chrono::Weekday, data: AnalysisDatum| {
        if weekday == analyzed_weekday && rotation == analyzed_rotation {
            aggregate += data;
        }
    };

    let func = |_coords: &CoverageCoordinates, coverage_and_workday: &mut CoverageAndWorkDay| {
        match &coverage_and_workday.coverages {
            Some(coverage) => match coverage {
                Coverage::Temporal(coverages) => {
                    for coverage in coverages {
                        let collection = coverage.collect_work(coverage_and_workday);
                        addfunc(coverage.get_rotation(), coverage.get_day(), collection);
                    }
                }
                Coverage::Fractional(coverages) => {
                    for coverage in coverages {
                        let collection = coverage.collect_work(coverage_and_workday);
                        addfunc(coverage.get_rotation(), coverage.get_day(), collection);
                    }
                }
            },
            None => {
                eprintln!("Uncovered work!");
                panic!("Uncovered work!");
            }
        }
    };

    coverage_map.foreach(func);

    Ok(aggregate)
}
