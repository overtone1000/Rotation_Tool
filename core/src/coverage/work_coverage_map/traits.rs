
use std::collections::{hash_map::Entry, HashMap};


use std::fmt::Debug;


use std::hash::Hash;







use crate::analysis::analysis_datum::{WorkUnit};

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

pub trait CoordinateMap<'a, T, U>
where
    T: 'a + Debug + Eq + PartialEq + Hash,
    U: Default + Debug,
{
    fn get_map(&mut self) -> &mut HashMap<T, U>;
    fn get_coordinate(coords: &CoverageCoordinates) -> T;
    fn get_branch(&'a mut self, coords: &'a CoverageCoordinates) -> &mut U {
        let key = Self::get_coordinate(coords);
        let retval = match (*self.get_map()).entry(key) {
            Entry::Occupied(o) => o.into_mut(),
            Entry::Vacant(v) => v.insert(U::default()),
        };
        retval
    }
}