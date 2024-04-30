use std::collections::HashSet;

use chrono::NaiveDate;

use crate::analysis::analysis_datum::AnalysisDatum;
use crate::analysis::volumes::VolumesMark;
use crate::rotations::description::WrappedSortable;
use crate::rotations::manifest::Manifest;
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
        self.get_branch_mut(coords).add_work(work);
    }
    fn add_coverage(
        &mut self,
        coords: &CoverageCoordinates,
        coverage: CoverageUnit,
    ) -> Result<(), Box<dyn std::error::Error>> {
        match &coverage {
            CoverageUnit::Temporal(_) => self.get_branch_mut(coords).add_coverage(coverage),
            CoverageUnit::WeekFraction(_) => {
                for weekday in ALL_DAYS {
                    let mut pseudocoords = coords.clone();
                    pseudocoords.weekday = **weekday;
                    match self
                        .get_branch_mut(&pseudocoords)
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
    fn clear_coverage(
        &mut self
    )->()
    {
        for branch in self.get_all_branches()
        {
            branch.clear_coverage();
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
        coords.context.to_string()
    }
}

type SubspecialtyMap = CoordinateMap<String, ContextMap>;
impl SpecifiedCoordinate<String> for SubspecialtyMap {
    fn get_coordinate(coords: &CoverageCoordinates) -> String {
        coords.subspecialty.to_string()
    }
}

pub type CoverageMap = CoordinateMap<String, SubspecialtyMap>;
impl SpecifiedCoordinate<String> for CoverageMap {
    fn get_coordinate(coords: &CoverageCoordinates) -> String {
        coords.facility.to_string()
    }
}

impl CoverageMap {
    pub fn foreach<T>(&self, mut func:T)->()
    where T:FnMut(&CoverageCoordinates, &CoverageAndWorkDay) {
        for (site, subspecialtymap) in self.get_map().iter() {
            for (subspecialty, contextmap) in subspecialtymap.get_map().iter() {
                for (context, weekdaymap) in contextmap.get_map().iter() {
                    //for (modality, weekdaymap) in modalitymap.map.iter_mut() {
                    for (weekday, coverage_and_workday) in weekdaymap.get_map().iter() {
                        let coords = CoverageCoordinates {
                            facility: site.to_string(),
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

    pub fn foreach_mut<T>(&mut self, mut func:T)->()
    where T:FnMut(&CoverageCoordinates, &mut CoverageAndWorkDay) {
        for (site, subspecialtymap) in self.get_map_mut().iter_mut() {
            for (subspecialty, contextmap) in subspecialtymap.get_map_mut().iter_mut() {
                for (context, weekdaymap) in contextmap.get_map_mut().iter_mut() {
                    //for (modality, weekdaymap) in modalitymap.map.iter_mut() {
                    for (weekday, coverage_and_workday) in weekdaymap.get_map_mut().iter_mut() {
                        let coords = CoverageCoordinates {
                            facility: site.to_string(),
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

    pub fn get_coverageandworkday<'a>(&'a self, coords:&'a CoverageCoordinates)->Option<&'a CoverageAndWorkDay>{
        let subspecialty_branch = self.get_branch(&coords)?;
        let context_branch = subspecialty_branch.get_branch(&coords)?;
        let weekday_branch = context_branch.get_branch(&coords)?;
        let coverage_and_work_day = weekday_branch.get_branch(&coords)?;
        
        Some(coverage_and_work_day)
    }


    pub fn populate_responsibility_volumes(
        &mut self,
        manifest: &mut Manifest,
        rotation_start:&NaiveDate,
        rotation_end:&NaiveDate
    ) -> Result<(), Box<dyn std::error::Error>> 
    {
        for rotation_description in &mut manifest.rotation_manifest {
            match rotation_description.responsibilities.get_mut() {
                Some(responsibilities) => {
                    for responsibility in responsibilities {                        
                        let mut dates:HashSet<NaiveDate>=HashSet::new();
                        let mut vm=VolumesMark{
                            rvu:0.0,
                            bvu:0.0
                        };

                        let coverages = CoverageMap::responsibility_to_coverages(rotation_description.rotation.as_str(), responsibility)?;
                    
                        for (coords, coverage) in coverages
                        {
                            self.clear_coverage();
                            self.add_coverage(&coords, coverage)?;
                            
                            self.foreach_mut(
                                |_coord:&CoverageCoordinates, coverage_and_workday:&mut CoverageAndWorkDay|
                                {
                                    coverage_and_workday.for_each_analysis_datum_by_rotation_date(
                                        |rotation_date:NaiveDate,ad:AnalysisDatum,_cu:&CoverageUnit|
                                        {
                                            if rotation_start<=&rotation_date && &rotation_date<=rotation_end
                                            {
                                                dates.insert(rotation_date);
                                                vm.rvu+=ad.get_rvu();
                                                vm.bvu+=ad.get_bvu();
                                            }
                                        }
                                    )
                                }
                            );
                        }

                        if dates.len()>0
                        {
                            vm.rvu/=f64::from(dates.len() as u32);
                            vm.bvu/=f64::from(dates.len() as u32);
                        }
                        responsibility.volume=Some(vm);

                
                    }
                }
                None => (),
            };
        }
        Ok(())
    }
}

impl JSONFileOut for CoverageMap {}
