use std::{collections::HashMap, error::Error, fs::File, io::BufWriter};

use chrono::NaiveDate;

use crate::{
    analysis::{
        analysis_datum::AnalysisDatum,
        volumes::{CategorizedVolumes, VolumesMark},
    },
    coverage::{
        coordinate::CoverageCoordinates, coverage_and_work_day::CoverageAndWorkDay,
        units::Coverage, work_collector::WorkCollector, work_coverage_map::maps::CoverageMap,
    },
};

pub fn sort_volumes_by_date(coverage_map: &mut CoverageMap) -> CategorizedVolumes {
    let mut retval: CategorizedVolumes = CategorizedVolumes::new();

    let mut process_collection =
        |date:NaiveDate, rotation: String, work: Vec<AnalysisDatum>| {
            for datum in work {
                let new_mark = VolumesMark {
                    rvu: datum.get_rvu(),
                    bvu: datum.get_bvu(),
                };
                retval.add(date, &rotation, new_mark);
            }
        };

    let func = |_coords: &CoverageCoordinates, coverage_and_workday: &mut CoverageAndWorkDay| {
        match &coverage_and_workday.coverages {
            Some(coverage) => {
                match coverage {
                    Coverage::Temporal(coverage) => {
                        for coverage_unit in coverage {
                            process_collection(
                                coverage_unit.get_rotation(),
                                coverage_unit.collect_work(coverage_and_workday),
                            );
                        }
                    }
                    Coverage::Fractional(coverage) => {
                        for coverage_unit in coverage {
                            process_collection(
                                coverage_unit.get_rotation(),
                                coverage_unit.collect_work(coverage_and_workday),
                            );
                        }
                    }
                };
            }
            None => {
                eprintln!("Uncovered work!");
                panic!("Uncovered work!");
            }
        }
    };

    coverage_map.foreach_mut(func);

    retval
}

pub fn analysis_to_plot(
    cat_vol: &mut CategorizedVolumes,
    filename: String,
) -> Result<(), Box<dyn Error>> {
    //let plot = self.analyze_by_day_of_week();
    let cachefile =
        File::create(&filename).unwrap_or_else(|_| panic!("Couldn't create file {}", &filename));
    let writer = BufWriter::new(&cachefile);
    serde_json::to_writer(writer, &cat_vol)?;
    Ok(())
}
