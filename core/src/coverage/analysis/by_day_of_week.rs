use std::{collections::{hash_map::Entry, BTreeSet, HashMap, HashSet}, fs::File};

use chrono::{Datelike, NaiveDate};

use crate::{
    analysis::analysis_datum::AnalysisDatum,
    coverage::{
        self, coordinate::CoverageCoordinates, coverage_and_work_day::{CoverageAndWorkDay, TimeAdjustment}, units::{Coverage, CoverageUnit}, work_coverage_map::maps::CoverageMap
    },
    globals::ALL_DAYS,
};

pub fn analyze_by_day_of_week(
    coverage_map: &CoverageMap,
) -> HashMap<String, HashMap<chrono::Weekday, AnalysisDatum>> {
    let mut retval: HashMap<String, HashMap<chrono::Weekday, AnalysisDatum>> = HashMap::new();
    let mut dates: HashMap<String,HashSet<NaiveDate>>=HashMap::new();
    
    let mut addfunc = |rotation: String, date: NaiveDate, data: AnalysisDatum| {

        let rotation_set=match dates.entry(rotation.to_string())
        {
            Entry::Occupied(occ) => {
                occ.into_mut()
            },
            Entry::Vacant(vac) => {
                vac.insert(HashSet::new())
            },
        };
        rotation_set.insert(date);

        let daymap: &mut HashMap<chrono::Weekday, AnalysisDatum> = match retval.entry(rotation) {
            Entry::Occupied(entry) => entry.into_mut(),
            Entry::Vacant(empty) => {
                let entry = empty.insert(HashMap::new());
                entry
            }
        };

        let datum: &mut AnalysisDatum = match daymap.entry(date.weekday()) {
            Entry::Occupied(entry) => entry.into_mut(),
            Entry::Vacant(empty) => {
                let entry = empty.insert(AnalysisDatum::default());
                entry
            }
        };

        *datum += data;
    };

    coverage_map.foreach(
        |_coords: &CoverageCoordinates, coverage_and_workday: &CoverageAndWorkDay|
        {
            coverage_and_workday.for_each_analysis_datum_by_rotation_date(
                |date:NaiveDate,ad:AnalysisDatum,cu:&CoverageUnit|
                {
                    addfunc(cu.get_rotation(),date,ad);
                }
            );
        }
    );

    for (rotation, rotation_data) in &mut retval
    {
        let dates=dates.get(rotation).expect("Shouldn't ever happen.");
        for (weekday, datum) in rotation_data
        {
            let weekday_count=dates.iter().filter(|date|{date.weekday()==*weekday}).count();
            let denominator:f64=f64::try_from(weekday_count as u32).expect("Invalid denominator");
            datum.scale(1.0/denominator);
        }
    }

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
