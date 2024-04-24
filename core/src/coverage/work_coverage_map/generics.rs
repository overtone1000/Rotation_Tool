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
    fn clear_coverage(&mut self)->();
}

#[derive(Debug, Serialize, Default, Clone)]
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
    pub fn get_branch(&'a self, coords: &'a CoverageCoordinates) -> Option<&U> {
        let key = Self::get_coordinate(coords);
        self.map.get(&key)
    }
    pub fn get_branch_mut(&'a mut self, coords: &'a CoverageCoordinates) -> &mut U {
        let key = Self::get_coordinate(coords);
        let retval = match self.map.entry(key) {
            Entry::Occupied(o) => o.into_mut(),
            Entry::Vacant(v) => v.insert(U::default()),
        };
        retval
    }
    pub fn get_all_branches(&mut self)-> Vec<&mut U>
    {
        return self.map.values_mut().into_iter().collect()
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
        self.get_branch_mut(coords).add_work(coords, work)
    }

    fn add_coverage(
        &mut self,
        coords: &CoverageCoordinates,
        coverage: CoverageUnit,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.get_branch_mut(coords).add_coverage(coords, coverage)
    }

    fn clear_coverage(
        &mut self
    )->()
    {
        for branch in self.get_all_branches(){
            branch.clear_coverage();
        }
    }
}

pub trait SpecifiedCoordinate<T> {
    fn get_coordinate(coords: &CoverageCoordinates) -> T;
}
