use std::collections::{HashMap, HashSet};
use std::str::FromStr;

use chrono::Datelike;

use crate::coverage::coordinate::CoverageCoordinates;

use crate::coverage::coverage_and_work_day;
use crate::coverage::units::fractional_coverage::FractionalCoverageUnit;
use crate::coverage::units::temporal_coverage::{weekday_plus, TemporalCoverageUnit};
use crate::coverage::units::CoverageUnit;

use crate::globals::{self};
use crate::rotations::description::WrappedSortable;
use crate::rotations::manifest::Manifest;
use crate::rotations::responsibility::RotationResponsibility;
use crate::rotations::rotation_error::RotationManifestParseError;

use super::generics::WorkCoverageMap;
use super::maps::CoverageMap;


impl CoverageMap {

    pub fn responsibility_to_coverages(rotation_name:&str,responsibility:&RotationResponsibility)->Result<Vec<(CoverageCoordinates,CoverageUnit)>, Box<dyn std::error::Error>>
    {
        let mut retval:Vec<(CoverageCoordinates,CoverageUnit)>=Vec::new();

        let all_weekday_strings: &[&str; 7] = &[
            &chrono::Weekday::Mon.to_string(),
            &chrono::Weekday::Tue.to_string(),
            &chrono::Weekday::Wed.to_string(),
            &chrono::Weekday::Thu.to_string(),
            &chrono::Weekday::Fri.to_string(),
            &chrono::Weekday::Sat.to_string(),
            &chrono::Weekday::Sun.to_string(),
        ];


        let mut coords: CoverageCoordinates = CoverageCoordinates::default();
        for site in responsibility.sites.to_vec(globals::FACILITIES) {
            coords.facility = site.to_string();
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
                        responsibility.days.to_vec(all_weekday_strings)
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

                                    //work_day_offset is the offset required to get from the day of the rotation to the work. So, PD (previous day) is -1.
                                    for (work_day_offset, start, end) in periods {
                                        coords.weekday =
                                            weekday_plus(weekday, work_day_offset);

                                        let coverage = TemporalCoverageUnit::create(
                                            start,
                                            end,
                                            rotation_name.to_string(),
                                            -work_day_offset, //this is the offset to get from the day of the rotation to the day of the work, so invert this value
                                        );

                                        retval.push(
                                            (coords.clone(),CoverageUnit::Temporal(coverage))
                                        );
                                    }
                                }
                            }
                            None => (),
                        }

                        match &responsibility.weekly_fraction {
                            Some(fraction) => {
                                coords.weekday = weekday;
                                let coverage = FractionalCoverageUnit::create(
                                    rotation_name.to_string(),
                                    weekday,
                                    fraction.to_owned(),
                                );
                                retval.push(
                                    (coords.clone(),CoverageUnit::WeekFraction(coverage))
                                );
                            }
                            None => (),
                        }
                    }
                    //}
                }
            }
        }
        Ok((retval))
    }

    pub fn add_coverage_from_manifest(
        &mut self,
        manifest: Manifest,
    ) -> Result<(), Box<dyn std::error::Error>> {
        for rotation_description in &manifest.rotation_manifest {
            match rotation_description.responsibilities.get() {
                Some(responsibilities) => {
                    for responsibility in responsibilities {
                        for (coords, coverage) in CoverageMap::responsibility_to_coverages(rotation_description.rotation.as_str(), responsibility)?
                        {
                            self.add_coverage(&coords, coverage)?;
                        }
                    }
                }
                None => (),
            };
        }

        Ok(())
    }
}
