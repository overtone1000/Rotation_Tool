use std::collections::{btree_map::Entry, BTreeMap, BTreeSet};

use chrono::NaiveDate;

use crate::{
    analysis::analysis_datum::{AnalysisDatum, ComparisonDatum},
    coverage::{
        coordinate::CoverageCoordinates, coverage_and_work_day::CoverageAndWorkDay,
        units::CoverageUnit, work_coverage_map::maps::CoverageMap,
    },
    serialization::output::JSONFileOut,
};

fn get_by_rotation_average(coverage_map: &CoverageMap) -> BTreeMap<String, ComparisonDatum> {
    let mut retval: BTreeMap<String, ComparisonDatum> = BTreeMap::new();
    let mut dates: BTreeMap<String, BTreeSet<NaiveDate>> = BTreeMap::new();

    let mut addfunc = |rotation: String, date: NaiveDate, data: AnalysisDatum| {
        let rotation_set = match dates.entry(rotation.to_string()) {
            Entry::Occupied(occ) => occ.into_mut(),
            Entry::Vacant(vac) => vac.insert(BTreeSet::new()),
        };
        rotation_set.insert(date);

        let datum: &mut ComparisonDatum = match retval.entry(rotation) {
            Entry::Occupied(entry) => entry.into_mut(),
            Entry::Vacant(empty) => {
                let entry = empty.insert(ComparisonDatum::default());
                entry
            }
        };

        datum.rvu += data.get_rvu();
        datum.bvu += data.get_bvu();
    };

    coverage_map.foreach(
        |_coords: &CoverageCoordinates, coverage_and_workday: &CoverageAndWorkDay| {
            coverage_and_workday.for_each_analysis_datum_by_rotation_date(
                |date: NaiveDate, ad: AnalysisDatum, cu: &CoverageUnit| {
                    addfunc(cu.get_rotation(), date, ad);
                },
            );
        },
    );

    for (rotation, datum) in &mut retval {
        let dates = dates.get(rotation).expect("Shouldn't ever happen.");
        let scale_factor: f64 =
            1.0 / (f64::try_from(dates.len() as u32).expect("Invalid denominator"));

        datum.rvu *= scale_factor;
        datum.bvu *= scale_factor;
    }

    retval
}

type ComparisonResult = BTreeMap<String, BTreeMap<String, ComparisonDatum>>;

pub fn compare(active_map: &CoverageMap, proposed_map: &CoverageMap) -> ComparisonResult {
    let active: BTreeMap<String, ComparisonDatum> = get_by_rotation_average(active_map);
    let proposed: BTreeMap<String, ComparisonDatum> = get_by_rotation_average(proposed_map);

    let mut retval: BTreeMap<String, BTreeMap<String, ComparisonDatum>> = BTreeMap::new();

    retval.insert("active".to_string(), active);
    retval.insert("proposed".to_string(), proposed);

    retval
}

impl JSONFileOut for ComparisonResult {}
