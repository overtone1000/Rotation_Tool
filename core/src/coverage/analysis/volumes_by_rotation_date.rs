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

pub fn sort_volumes_by_rotation_date(coverage_map: &CoverageMap) -> CategorizedVolumes {
    let mut retval: CategorizedVolumes = CategorizedVolumes::new();

    coverage_map.foreach(
        |_coords: &CoverageCoordinates, coverage_and_workday: &CoverageAndWorkDay| {
            coverage_and_workday.for_each_analysis_datum_by_rotation_date(
                |date:NaiveDate,ad:AnalysisDatum,cu:&CoverageUnit|
                {
                    let new_mark = VolumesMark {
                        rvu: ad.get_rvu(),
                        bvu: ad.get_bvu(),
                    };
                    retval.add(date, cu.get_rotation().as_str(), new_mark);
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
