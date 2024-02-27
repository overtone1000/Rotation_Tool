use std::collections::{hash_map::Entry, HashMap};

use crate::{
    analysis::analysis_datum::AnalysisDatum,
    coverage::{
        coordinate::CoverageCoordinates, coverage_and_work_day::CoverageAndWorkDay,
        units::Coverage, work_collector::WorkCollector, work_coverage_map::maps::CoverageMap,
    },
    globals::ALL_DAYS,
};

pub fn analyze_by_day_of_week(
    coverage_map: &mut CoverageMap,
) -> HashMap<String, HashMap<chrono::Weekday, AnalysisDatum>> {
    let mut retval: HashMap<String, HashMap<chrono::Weekday, AnalysisDatum>> = HashMap::new();

    let mut addfunc = |rotation: String, weekday: chrono::Weekday, data: AnalysisDatum| {
        let daymap: &mut HashMap<chrono::Weekday, AnalysisDatum> = match retval.entry(rotation) {
            Entry::Occupied(entry) => entry.into_mut(),
            Entry::Vacant(empty) => {
                let entry = empty.insert(HashMap::new());
                entry
            }
        };

        let datum: &mut AnalysisDatum = match daymap.entry(weekday) {
            Entry::Occupied(entry) => entry.into_mut(),
            Entry::Vacant(empty) => {
                let entry = empty.insert(AnalysisDatum::default());
                entry
            }
        };

        *datum += data;
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

    retval
}

pub fn analysis_to_csv(
    analysis: &HashMap<String, HashMap<chrono::Weekday, AnalysisDatum>>,
    path: String,
    use_rvu: bool,
) {
    let mut writer = match csv::WriterBuilder::new()
        .delimiter(b',')
        .quote(b'"')
        .has_headers(false) //write manually
        .from_path(path)
    {
        Ok(x) => x,
        Err(_) => {
            panic!();
        }
    };

    let mut headers: Vec<String> = Vec::new();
    headers.push("".to_string());
    for weekday in ALL_DAYS {
        headers.push(weekday.to_string());
    }
    match writer.write_record(headers) {
        Ok(_) => (),
        Err(_) => panic!(),
    }

    let mut rotations: Vec<String> = Vec::new();
    for key in analysis.keys() {
        rotations.push(key.to_string());
    }
    rotations.sort();

    for rotation in &rotations {
        let daymap = analysis.get(rotation).expect("Should be a key");
        let mut v: Vec<String> = Vec::new();
        v.push(rotation.to_string());

        for weekday in ALL_DAYS {
            let val = match daymap.get(weekday) {
                Some(x) => {
                    if use_rvu {
                        x.get_rvu().to_string()
                    } else {
                        x.get_bvu().to_string()
                    }
                }
                None => "".to_string(),
            };
            v.push(val);
        }

        match writer.write_record(v) {
            Ok(_) => (),
            Err(_) => panic!(),
        }
    }
}
