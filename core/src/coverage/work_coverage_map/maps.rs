use chrono::Datelike;

use crate::analysis::analysis_datum::WorkUnit;

use crate::coverage::coordinate::CoverageCoordinates;
use crate::coverage::coverage_and_work_day::CoverageAndWorkDay;

use crate::coverage::units::CoverageUnit;

use crate::globals::ALL_DAYS;

use crate::serialization::output::JSONFileOut;
use crate::serialization::weekday::SerializeableWeekday;

use super::generics::{CoordinateMap, SpecifiedCoordinate, WorkCoverageMap};

type WeekdayMap = CoordinateMap<SerializeableWeekday, CoverageAndWorkDay>;
//pub struct WeekdayMap {
//    map: HashMap<SerializeableWeekday, CoverageAndWorkDay>,
//}

impl WorkCoverageMap for WeekdayMap {
    fn add_work(&mut self, coords: &CoverageCoordinates, work: WorkUnit) {
        self.get_branch(coords).add_work(work);
    }
    fn add_coverage(
        &mut self,
        coords: &CoverageCoordinates,
        coverage: CoverageUnit,
    ) -> Result<(), Box<dyn std::error::Error>> {
        match &coverage {
            CoverageUnit::Temporal(_) => self.get_branch(coords).add_coverage(coverage),
            CoverageUnit::WeekFraction(_) => {
                for weekday in ALL_DAYS {
                    let mut pseudocoords = coords.clone();
                    pseudocoords.weekday = **weekday;
                    match self
                        .get_branch(&pseudocoords)
                        .add_coverage(coverage.to_owned())
                    {
                        Ok(_) => (),
                        Err(e) => {
                            return Err(e);
                        }
                    }
                }
                Ok(())
            }
        }
    }
}

impl SpecifiedCoordinate<SerializeableWeekday> for WeekdayMap {
    fn get_coordinate(coords: &CoverageCoordinates) -> SerializeableWeekday {
        SerializeableWeekday {
            day: coords.weekday,
        }
    }
}

type ContextMap = CoordinateMap<String, WeekdayMap>;
impl SpecifiedCoordinate<String> for ContextMap {
    fn get_coordinate(coords: &CoverageCoordinates) -> String {
        coords.context.clone()
    }
}

type SubspecialtyMap = CoordinateMap<String, ContextMap>;
impl SpecifiedCoordinate<String> for SubspecialtyMap {
    fn get_coordinate(coords: &CoverageCoordinates) -> String {
        coords.subspecialty.clone()
    }
}

pub type CoverageMap = CoordinateMap<String, SubspecialtyMap>;
impl SpecifiedCoordinate<String> for CoverageMap {
    fn get_coordinate(coords: &CoverageCoordinates) -> String {
        coords.site.clone()
    }
}

impl CoverageMap {
    pub fn foreach(&mut self, mut func: impl FnMut(&CoverageCoordinates, &mut CoverageAndWorkDay)) {
        for (site, subspecialtymap) in self.get_map().iter_mut() {
            for (subspecialty, contextmap) in subspecialtymap.get_map().iter_mut() {
                for (context, weekdaymap) in contextmap.get_map().iter_mut() {
                    //for (modality, weekdaymap) in modalitymap.map.iter_mut() {
                    for (weekday, coverage_and_workday) in weekdaymap.get_map().iter_mut() {
                        let coords = CoverageCoordinates {
                            site: site.to_string(),
                            subspecialty: subspecialty.to_string(),
                            context: context.to_string(),
                            //modality: modality.to_string(),
                            weekday: weekday.day,
                        };

                        func(&coords, coverage_and_workday);
                    }
                    //}
                }
            }
        }
    }
}

impl JSONFileOut for CoverageMap {}
