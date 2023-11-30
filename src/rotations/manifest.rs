use std::{arch::x86_64, fmt};
use std::collections::BTreeMap;
use std::error::Error;
use std::fs;
use std::io::Read;
use chrono::NaiveTime;
use serde::de;
use serde::{Serialize, Deserialize, de::Visitor};

use crate::globals::file_names::EXAMPLE_ROTATION_DESCRIPTIONS;

use super::baseline::RotationBaseline;
use super::description::RotationDescription;
use super::responsibility::{RotationResponsibility, self};
use super::special::{self, weekdays};
use super::stringtypes::{StringTypes, SlashSeparatedStringVec};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Manifest
{
    pub title:String,
    pub rotation_manifest:Vec<RotationDescription>,
    pub baselines:Option<Vec<RotationBaseline>>
}

impl Manifest
{
    pub fn parse(filename:&str)->Result<Manifest, Box<dyn Error>>
    {
        let rdr = fs::File::open(filename)?;
        let retval:Manifest=serde_yaml::from_reader(rdr)?;

        let mut noerrs = true;
        for desc in &retval.rotation_manifest
        {
            match &desc.responsibilities
{
                Some(responsibilities)=>{
                    for resp in responsibilities
            {
                match resp.validate()
                {
                    Err(x) => {
                        noerrs=false;
                        for e in x
                        {
                            eprintln!("Error in {} rotation. {}",desc.rotation,e);
                        }
                    },
                    _=>()
                }
            }
},
                None=>()
            };
        }

        if noerrs
        {
            Ok(retval)
        }
        else {
            Err("Malformed manifest".into())
        }
    }

    pub fn create_example()->Result<(), Box<dyn Error>>
    {
        let mut example=Manifest{
            title:"Rotation Description Example".to_string(),
            rotation_manifest:Vec::new(),
            baselines:None
        };

        example.rotation_manifest.push(
            RotationDescription { 
                rotation: "Rotation A".to_string(),
                location: "Rot A Location".to_string(),
                responsibilities: Some(vec![
                    RotationResponsibility{
                        sites:StringTypes::new_slash_separated_string_vec("Site 1/Site 2"),
                        subspecialties:StringTypes::new_slash_separated_string_vec("Subspecialty 1/Subspecialty 2"),
                        contexts:StringTypes::new_slash_separated_string_vec("Context 1/Context 2"),
                        modalities:StringTypes::new_slash_separated_string_vec("Modality 1/Modality 2"),
                        time_periods:Some(StringTypes::Array(vec!["17:00 PBD-12:00 CD".to_string(), "13:00 CD-17:00 CD".to_string()])),
                        weekly_fraction:None,
                        days:
                            StringTypes::new_slash_separated_string_vec(
                                &(weekdays::weekday_to_str(chrono::Weekday::Mon)+"/"+
                                &weekdays::weekday_to_str(chrono::Weekday::Tue)+"/"+
                                &weekdays::weekday_to_str(chrono::Weekday::Wed)+"/"+
                                &weekdays::weekday_to_str(chrono::Weekday::Thu)+"/"+
                                &weekdays::weekday_to_str(chrono::Weekday::Fri))
                            ),
                    },
                    RotationResponsibility{
                        sites:StringTypes::Array(vec!["Site A".to_string(),"Site B".to_string()]),
                        subspecialties:StringTypes::Array(vec!["Specialty A".to_string(),"Specialty B".to_string()]),
                        contexts:StringTypes::Array(vec!["Context A".to_string(),"Context B".to_string()]),
                        modalities:StringTypes::Array(vec!["Modality A".to_string(),"Modality B".to_string()]),
                        time_periods:Some(StringTypes::Array(vec!["17:00 PD-12:00 CD".to_string(), "13:00 CD-17:00 CD".to_string()])),
                        weekly_fraction:None,
                        days:StringTypes::Array(vec![
                            weekdays::weekday_to_str(chrono::Weekday::Sat),
                            weekdays::weekday_to_str(chrono::Weekday::Sun)
                            ]
                        )
                    }
                ]),
                comments:Some(vec!("Comments can go here.".to_string(),"Comments are an array.".to_string(),"But this section can be omitted entirely.".to_string()))
            }
        );

        let baselines = vec![
            RotationBaseline { 
                rotation: "Rotation A".to_string(),
                RVU: 71.2,
                BVU: 2100.2
            }
        ];

        example.baselines=Some(baselines);

        let writer = fs::File::create(EXAMPLE_ROTATION_DESCRIPTIONS)?;
        
        serde_yaml::to_writer(writer, &example)?;
        Ok(())
    }
}