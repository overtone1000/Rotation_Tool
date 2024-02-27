use std::collections::HashSet;
use std::collections::{hash_map::Entry, HashMap};

use std::fmt::Debug;

use std::str::FromStr;

use chrono::{Datelike, Duration, NaiveDate, NaiveDateTime, Weekday};
use serde::Serialize;

use crate::analysis::analysis_datum::WorkUnit;

use crate::coverage::coordinate::CoverageCoordinates;
use crate::coverage::coverage_and_work_day::CoverageAndWorkDay;
use crate::coverage::distribution::get_normal_dist_weights;

use crate::coverage::units::fractional_coverage::FractionalCoverageUnit;
use crate::coverage::units::temporal_coverage::{weekday_plus, TemporalCoverageUnit};
use crate::coverage::units::CoverageUnit;

use crate::globals::{self, ALL_DAYS};
use crate::rotations::description::WrappedSortable;
use crate::rotations::manifest::Manifest;
use crate::rotations::rotation_error::RotationManifestParseError;

use crate::error::source_error::SourceError;
use crate::serialization::output::JSONFileOut;
use crate::serialization::weekday::SerializeableWeekday;
use crate::source_data::processing::categorization::{
    build_salem_bvumap, build_salem_rvumap, get_categories_map,
};
use crate::source_data::processing::processed_source::ProcessedSource;
use crate::{
    constraints::ConstraintSet,
    dates::BUSINESS_DAYS_PER_YEAR,
    globals::{main_headers, tpc_headers, BUSINESS_DAYS, SITES},
};

use super::generics::{CoordinateMap, SpecifiedCoordinate, WorkCoverageMap};
use super::maps::CoverageMap;
impl CoverageMap {
    pub fn add_coverage_from_manifest(
        &mut self,
        manifest: Manifest,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let all_weekdays_strings: &[&str; 7] = &[
            &chrono::Weekday::Mon.to_string(),
            &chrono::Weekday::Tue.to_string(),
            &chrono::Weekday::Wed.to_string(),
            &chrono::Weekday::Thu.to_string(),
            &chrono::Weekday::Fri.to_string(),
            &chrono::Weekday::Sat.to_string(),
            &chrono::Weekday::Sun.to_string(),
        ];

        let mut coords: CoverageCoordinates = CoverageCoordinates::default();

        for rotation_description in &manifest.rotation_manifest {
            match &rotation_description.responsibilities.get() {
                Some(responsibilities) => {
                    for responsibility in responsibilities {
                        for site in responsibility.sites.to_vec(globals::SITES) {
                            coords.site = site.to_string();
                            for subspecialty in responsibility.exams.to_vec(globals::SUBSPECIALTIES)
                            {
                                coords.subspecialty = subspecialty.to_string();
                                for context in responsibility.contexts.to_vec(globals::CONTEXTS) {
                                    coords.context = context.to_string();
                                    //for modality in
                                    //    responsibility.modalities.to_vec(globals::MODALITIES)
                                    //{
                                    //coords.modality = modality.to_string();
                                    for weekday_string in
                                        responsibility.days.to_vec(all_weekdays_strings)
                                    {
                                        let weekday = match chrono::Weekday::from_str(
                                            &weekday_string,
                                        ) {
                                            Ok(x) => x,
                                            Err(_) => {
                                                return RotationManifestParseError::generate_boxed(
                                                    0,
                                                    "".to_string(),
                                                )
                                            }
                                        };

                                        if responsibility.time_periods.get().is_some()
                                            && responsibility.weekly_fraction.is_some()
                                        {
                                            return RotationManifestParseError::generate_boxed(0,"'time_periods' and 'fraction' have both been provided. One and only one must be provided.".to_string());
                                        }
                                        if responsibility.time_periods.get().is_none()
                                            && responsibility.weekly_fraction.is_none()
                                        {
                                            return RotationManifestParseError::generate_boxed(
                                                0,
                                                "Neither 'time_periods' nor 'fraction' provided."
                                                    .to_string(),
                                            );
                                        }

                                        match &responsibility.time_periods.get() {
                                            Some(time_periods) => {
                                                for time_period in time_periods {
                                                    /*
                                                    let timespan =
                                                        parse_time_span(time_period.as_str())
                                                            .expect(
                                                            "Erroneous timespan in manifest.",
                                                        );
                                                    */
                                                    let periods =
                                                        time_period.instantiate_periods(weekday);

                                                    for (day_offset, start, end) in periods {
                                                        coords.weekday =
                                                            weekday_plus(weekday, day_offset);

                                                        let coverage = TemporalCoverageUnit::create(
                                                            start,
                                                            end,
                                                            rotation_description
                                                                .rotation
                                                                .to_string(),
                                                            weekday, //the NOMINAL weekday
                                                        );

                                                        match self.add_coverage(
                                                            &coords,
                                                            CoverageUnit::Temporal(coverage),
                                                        ) {
                                                            Ok(_) => (),
                                                            Err(e) => {
                                                                return Err(e);
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                            None => (),
                                        }

                                        match &responsibility.weekly_fraction {
                                            Some(fraction) => {
                                                coords.weekday = weekday;
                                                let coverage = FractionalCoverageUnit::create(
                                                    rotation_description.rotation.to_string(),
                                                    weekday,
                                                    fraction.to_owned(),
                                                );
                                                self.add_coverage(
                                                    &coords,
                                                    CoverageUnit::WeekFraction(coverage),
                                                )?;
                                            }
                                            None => (),
                                        }
                                    }
                                    //}
                                }
                            }
                        }
                    }
                }
                None => (),
            };
        }

        Ok(())
    }
}