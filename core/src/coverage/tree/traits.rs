use std::collections::HashSet;
use std::collections::{hash_map::Entry, HashMap};

use std::error::Error;
use std::fmt::Debug;

use std::fs::File;
use std::hash::Hash;
use std::io::{BufWriter, Write};
use std::ops::AddAssign;
use std::str::FromStr;

use chrono::{Datelike, Duration, NaiveDate, NaiveDateTime, Timelike};
use serde::Serialize;

use crate::analysis::analysis_datum::{AnalysisDatum, WorkUnit};
use crate::analysis::volumes::{CategorizedVolumes, VolumesMark};
use crate::coverage::coordinate::CoverageCoordinates;
use crate::coverage::coverage_and_work_day::CoverageAndWorkDay;
use crate::coverage::units::temporal_coverage::{weekday_plus, TemporalCoverageUnit};
use crate::coverage::units::CoverageUnit;
use crate::globals::{self, ALL_DAYS};
use crate::output::JSONable;
use crate::rotations::description::WrappedSortable;
use crate::rotations::manifest::{Manifest};
use crate::rotations::rotation_error::RotationManifestParseError;
use crate::rotations::time_modifiers::{NEXT_MIDNIGHT, THIS_MIDNIGHT, TimeSinceMidnight};
use crate::source_data::processing::categorization::{build_salem_bvumap, build_salem_rvumap, get_categories_map};
use crate::source_data::processing::processed_source::ProcessedSource;
use crate::error::source_error::SourceError;
use crate::{
    constraints::ConstraintSet,
    dates::BUSINESS_DAYS_PER_YEAR,
    globals::{main_headers, tpc_headers, BUSINESS_DAYS, SITES},
};


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