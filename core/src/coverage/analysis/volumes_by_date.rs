use std::{collections::HashMap, error::Error, fs::File, io::BufWriter};

use chrono::NaiveDate;

use crate::{
    analysis::{
        analysis_datum::AnalysisDatum,
        volumes::{CategorizedVolumes, VolumesMark},
    },
    coverage::{
        self, coordinate::CoverageCoordinates, coverage_and_work_day::{CoverageAndWorkDay, TimeAdjustment}, units::{Coverage, CoverageUnit}, work_coverage_map::maps::CoverageMap
    },
};

pub fn sort_volumes_by_date(coverage_map: &mut CoverageMap) -> CategorizedVolumes {
    let mut retval: CategorizedVolumes = CategorizedVolumes::new();

    let mut process_datum =
        |date:NaiveDate, rotation: String, work: AnalysisDatum| {
            let new_mark = VolumesMark {
                rvu: work.get_rvu(),
                bvu: work.get_bvu(),
            };
            retval.add(date, &rotation, new_mark);
        };

    coverage_map.foreach_mut(
        |coords: &CoverageCoordinates, coverage_and_workday: &mut CoverageAndWorkDay| {
            coverage_and_workday.for_each_analysis_datum_by_rotation_date(
                |date:NaiveDate,ad:AnalysisDatum,cu:&CoverageUnit|
                {
                    process_datum(date,cu.get_rotation(),ad);
                }
            )
        }
    );

    println!("{:?}",retval);

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
