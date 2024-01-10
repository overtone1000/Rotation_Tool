

use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::error::Error;
use std::fs::{self, File};
use std::io::BufWriter;



use crate::globals::file_names::EXAMPLE_ROTATION_DESCRIPTIONS;

use super::baseline::RotationBaseline;
use super::description::{RotationDescription, WrappedSortable, Responsibilities, RotationHours};
use super::responsibility::{RotationResponsibility, TimePeriods};
use super::special::weekdays;
use super::stringtypes::StringTypes;
use super::time_modifiers::{RelativeTime, TimeSinceMidnight};
use super::timespan::Timespan;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Manifest {
    pub title: String,
    pub rotation_manifest: Vec<RotationDescription>,
    pub baselines: Option<Vec<RotationBaseline>>,
}

impl Manifest {
    pub fn parse(filename: &str) -> Result<Manifest, Box<dyn Error>> {
        let rdr = fs::File::open(filename)?;
        let retval: Manifest = serde_yaml::from_reader(rdr)?;

        let mut noerrs = true;
        for desc in &retval.rotation_manifest {
            match &desc.responsibilities.get() {
                Some(responsibilities) => {
                    for resp in responsibilities {
                        match resp.validate() {
                            Err(x) => {
                                noerrs = false;
                                for e in x {
                                    eprintln!("Error in {} rotation. {}", desc.rotation, e);
                                }
                            }
                            _ => (),
                        }
                    }
                }
                None => (),
            };
        }

        if noerrs {
            Ok(retval)
        } else {
            Err("Malformed manifest".into())
        }
    }

    pub fn create_example() -> Result<(), Box<dyn Error>> {
        let mut example = Manifest {
            title: "Rotation Description Example".to_string(),
            rotation_manifest: Vec::new(),
            baselines: None,
        };

        example.rotation_manifest.push(RotationDescription {
            rotation: "Rotation A".to_string(),
            location: "Rot A Location".to_string(),
            responsibilities: Responsibilities::from_vec(vec![
                RotationResponsibility {
                    sites: StringTypes::new_slash_separated_string_vec("Site 1/Site 2"),
                    subspecialties: StringTypes::new_slash_separated_string_vec(
                        "Subspecialty 1/Subspecialty 2",
                    ),
                    contexts: StringTypes::new_slash_separated_string_vec("Context 1/Context 2"),
                    modalities: StringTypes::new_slash_separated_string_vec(
                        "Modality 1/Modality 2",
                    ),
                    time_periods: TimePeriods::from_strings(Vec::from([
                        "17:00 PBD-12:00 CD",
                        "13:00 CD-17:00 CD",
                    ])).expect("Should be valid."),
                    weekly_fraction: None,
                    days: StringTypes::new_slash_separated_string_vec(
                        &(weekdays::weekday_to_str(chrono::Weekday::Mon)
                            + "/"
                            + &weekdays::weekday_to_str(chrono::Weekday::Tue)
                            + "/"
                            + &weekdays::weekday_to_str(chrono::Weekday::Wed)
                            + "/"
                            + &weekdays::weekday_to_str(chrono::Weekday::Thu)
                            + "/"
                            + &weekdays::weekday_to_str(chrono::Weekday::Fri)),
                    ),
                },
                RotationResponsibility {
                    sites: StringTypes::Array(HashSet::from([
                        "Site A".to_string(),
                        "Site B".to_string(),
                    ])),
                    subspecialties: StringTypes::Array(HashSet::from([
                        "Specialty A".to_string(),
                        "Specialty B".to_string(),
                    ])),
                    contexts: StringTypes::Array(HashSet::from([
                        "Context A".to_string(),
                        "Context B".to_string(),
                    ])),
                    modalities: StringTypes::Array(HashSet::from([
                        "Modality A".to_string(),
                        "Modality B".to_string(),
                    ])),
                    time_periods:  TimePeriods::from_strings(Vec::from([
                        "17:00 PD-12:00 CD",
                        "13:00 CD-17:00 CD",
                    ])).expect("Should be valid."),
                    weekly_fraction: None,
                    days: StringTypes::Array(HashSet::from([
                        weekdays::weekday_to_str(chrono::Weekday::Sat),
                        weekdays::weekday_to_str(chrono::Weekday::Sun),
                    ])),
                },
            ]),
            comments: Some(HashSet::from([
                "Comments can go here.".to_string(),
                "Comments are an array.".to_string(),
                "But this section can be omitted entirely.".to_string(),
            ])),
            hours: Some(Vec::from([RotationHours::new(
                RelativeTime::CurrentDay(TimeSinceMidnight::new(8*60)),
                RelativeTime::CurrentDay(TimeSinceMidnight::new(17*60)),
                StringTypes::new_slash_separated_string_vec("Sun/Mon/Tue/Wed/Thu/Fri")
            )])),
            breaktime: Some((
                Timespan{
                    start:RelativeTime::CurrentDay(TimeSinceMidnight::new(12*60)),
                    stop:RelativeTime::CurrentDay(TimeSinceMidnight::new(13*60))
                }
                ,
                Some("Covered by Rotation C".to_string())
            )),
        });

        let baselines = vec![RotationBaseline {
            rotation: "Rotation A".to_string(),
            rvu: 71.2,
            bvu: 2100.2,
        }];

        example.baselines = Some(baselines);

        let writer = fs::File::create(EXAMPLE_ROTATION_DESCRIPTIONS)?;

        serde_yaml::to_writer(writer, &example)?;
        Ok(())
    }
}

pub trait JSONable:Serialize
{
    fn to_json(&self, filename:&str) -> Result<(), Box<dyn Error>> {
        let file = File::create(filename)?;
        let writer = BufWriter::new(file);
        serde_json::to_writer(writer, self)?;
        Ok(())
    }
}

impl JSONable for Manifest {
}