use crate::coverage::coordinate::{Context, Site, Subspecialty};
use crate::{analysis::analysis_datum::WorkUnit, coverage::coordinate::CoverageCoordinates};
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

type ContextMap = CoordinateMap<Context, WeekdayMap>;
impl SpecifiedCoordinate<Context> for ContextMap {
    fn get_coordinate(coords: &CoverageCoordinates) -> Context {
        coords.context
    }
}

type SubspecialtyMap = CoordinateMap<Subspecialty, ContextMap>;
impl SpecifiedCoordinate<Subspecialty> for SubspecialtyMap {
    fn get_coordinate(coords: &CoverageCoordinates) -> Subspecialty {
        coords.subspecialty
    }
}

pub type CoverageMap = CoordinateMap<Site, SubspecialtyMap>;
impl SpecifiedCoordinate<Site> for CoverageMap {
    fn get_coordinate(coords: &CoverageCoordinates) -> Site {
        coords.site
    }
}

impl CoverageMap {
    pub fn foreach_mut(&mut self, mut func: impl FnMut(&CoverageCoordinates, &mut CoverageAndWorkDay)) {
        for (site, subspecialtymap) in self.get_map_mut().iter_mut() {
            for (subspecialty, contextmap) in subspecialtymap.get_map_mut().iter_mut() {
                for (context, weekdaymap) in contextmap.get_map_mut().iter_mut() {
                    //for (modality, weekdaymap) in modalitymap.map.iter_mut() {
                    for (weekday, coverage_and_workday) in weekdaymap.get_map_mut().iter_mut() {
                        let coords = CoverageCoordinates {
                            site: *site,
                            subspecialty: *subspecialty,
                            context: *context,
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
