use std::collections::{hash_map::Entry, HashMap};

use std::fmt::Debug;

use std::hash::Hash;

use serde::Serialize;

use crate::analysis::analysis_datum::WorkUnit;

use crate::coverage::coordinate::CoverageCoordinates;

use crate::coverage::units::CoverageUnit;

pub trait WorkCoverageMap {
    fn add_work(&mut self, coords: &CoverageCoordinates, work: WorkUnit);
    fn add_coverage(
        &mut self,
        coords: &CoverageCoordinates,
        coverage: CoverageUnit,
    ) -> Result<(), Box<dyn std::error::Error>>;
}

#[derive(Debug, Serialize, Default)]
pub struct CoordinateMap<T, U>
where
    T: Debug + Eq + PartialEq + Hash,
    U: Default + Debug,
{
    map: HashMap<T, U>,
}

impl<'a, T, U> CoordinateMap<T, U>
where
    CoordinateMap<T, U>: SpecifiedCoordinate<T>,
    T: 'a + Debug + Eq + PartialEq + Hash,
    U: Default + Debug,
{
    pub fn get_map(&self) -> &HashMap<T, U> {
        &self.map
    }
    pub fn get_map_mut(&mut self) -> &mut HashMap<T, U> {
        &mut self.map
    }
    pub fn get_branch(&'a mut self, coords: &'a CoverageCoordinates) -> &mut U {
        let key = Self::get_coordinate(coords);
        let retval = match self.map.entry(key) {
            Entry::Occupied(o) => o.into_mut(),
            Entry::Vacant(v) => v.insert(U::default()),
        };
        retval
    }
}

//Default impl, overridden for WorkCoverageMap to handle pseudocoords for week fractions
impl<T, U> WorkCoverageMap for CoordinateMap<T, U>
where
    CoordinateMap<T, U>: SpecifiedCoordinate<T>,
    T: Debug + Eq + PartialEq + Hash,
    U: Default + Debug + WorkCoverageMap,
{
    fn add_work(&mut self, coords: &CoverageCoordinates, work: WorkUnit) {
        self.get_branch(coords).add_work(coords, work)
    }

    fn add_coverage(
        &mut self,
        coords: &CoverageCoordinates,
        coverage: CoverageUnit,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.get_branch(coords).add_coverage(coords, coverage)
    }
}

pub trait SpecifiedCoordinate<T> {
    fn get_coordinate(coords: &CoverageCoordinates) -> T;
}
