use std::{collections::{hash_map::Entry, HashMap}, error::Error, fs::File, io::BufWriter};

use chrono::NaiveDate;

use crate::{analysis::{analysis_datum::{AnalysisDatum, SerializeableNaiveDateTime}, volumes::{CategorizedVolumes, VolumesMark}}, coverage::{coordinate::CoverageCoordinates, coverage_and_work_day::CoverageAndWorkDay, units::Coverage, work_collector::WorkCollector, work_coverage_map::CoverageMap}, globals::ALL_DAYS};

impl CoverageMap
{
pub fn sort_volumes_by_date(&mut self) -> CategorizedVolumes {
    let mut retval: CategorizedVolumes = CategorizedVolumes::new();

    let mut process_collection =
        |rotation:String, collected_by_date:HashMap<NaiveDate,AnalysisDatum>| {
            for (date, datum) in collected_by_date
            {
                let new_mark=VolumesMark{
                    rvu:datum.get_rvu(),
                    bvu:datum.get_bvu()
                };
                retval.add(date, &rotation, new_mark);
            }
        };

    let func =
        |_coords: &CoverageCoordinates, coverage_and_workday: &mut CoverageAndWorkDay| {
            match &coverage_and_workday.coverages {
                Some(coverage) => {
                    match coverage {
                        Coverage::Temporal(coverage) => {
                            for coverage_unit in coverage
                            {
                                process_collection(coverage_unit.get_rotation(), coverage_unit.collect_work_bydate(coverage_and_workday));
                            }
                        },
                        Coverage::Fractional(coverage) => {
                            for coverage_unit in coverage
                            {
                                process_collection(coverage_unit.get_rotation(), coverage_unit.collect_work_bydate(coverage_and_workday));
                            }
                        },
                    };
                },
                None => {
                    eprintln!("Uncovered work!");
                    panic!("Uncovered work!");
                }
            }
        };

    self.foreach(func);

    retval
}

pub fn analysis_to_plot(&mut self, filename: String) -> Result<(), Box<dyn Error>>  {
    let plot=self.sort_volumes_by_date();
    //let plot = self.analyze_by_day_of_week();
    let cachefile = File::create(&filename).expect(format!("Couldn't create file {}",&filename).as_str());
    let writer = BufWriter::new(&cachefile);
    serde_json::to_writer(writer,&plot)?;
    Ok(())
}
}