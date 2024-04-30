use std::{collections::BTreeMap, error::Error, fs::File, io::BufWriter};

use chrono::NaiveDate;

use crate::{analysis::volumes::VolumesMark, coverage::{coordinate::CoverageCoordinates, coverage_and_work_day::CoverageAndWorkDay, work_coverage_map::maps::CoverageMap}};

pub type VolBySiteAndDate = BTreeMap<NaiveDate,BTreeMap<String,VolumesMark>>;

pub(crate) fn sort_volumes_by_facility_and_date(coverage_map: &mut CoverageMap) -> VolBySiteAndDate {
    let mut retval: VolBySiteAndDate = BTreeMap::new();

    coverage_map.foreach_mut(
        |coords: &CoverageCoordinates, coverage_and_workday: &mut CoverageAndWorkDay| {
            for wu in coverage_and_workday.work_iterator()
            {
                let new_mark = VolumesMark {
                    rvu: wu.get_absolute_rvu(),
                    bvu: wu.get_absolute_bvu(),
                };
    
                let site_map = match retval.entry(wu.get_datetime().date())
                {
                    std::collections::btree_map::Entry::Vacant(vac) => vac.insert(BTreeMap::new()),
                    std::collections::btree_map::Entry::Occupied(occ) => occ.into_mut(),
                };
                match site_map.entry(coords.facility.to_string())
                {
                    std::collections::btree_map::Entry::Vacant(vac) => {vac.insert(new_mark);},
                    std::collections::btree_map::Entry::Occupied(occ) => {*occ.into_mut()+=new_mark;},
                }
            }
        }
    );

    println!("{:?}",retval);

    retval
}

pub fn volumes_by_facility_and_date_to_plot(
    cat_vol: &mut VolBySiteAndDate,
    filename: String,
) -> Result<(), Box<dyn Error>> {
    //let plot = self.analyze_by_day_of_week();
    let cachefile =
        File::create(&filename).unwrap_or_else(|_| panic!("Couldn't create file {}", &filename));
    let writer = BufWriter::new(&cachefile);
    serde_json::to_writer(writer, &cat_vol)?;
    Ok(())
}
