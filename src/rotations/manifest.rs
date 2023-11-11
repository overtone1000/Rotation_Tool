use std::{arch::x86_64, fmt};
use std::collections::BTreeMap;
use std::error::Error;
use std::fs;
use std::io::Read;
use chrono::NaiveTime;
use serde::de;
use serde::{Serialize, Deserialize, de::Visitor};

use crate::globals::file_names::EXAMPLE_ROTATION_DESCRIPTIONS;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Manifest
{
    title:String,
    descriptions:Vec<RotationDescription>,
    baselines:Vec<RotationBaseline>
}

impl Manifest
{
    pub fn parse(filename:&String)->Result<Manifest, Box<dyn Error>>
    {
        let rdr = fs::File::open(filename)?;
        let retval:Manifest=serde_yaml::from_reader(rdr)?;

        Ok(retval)
    }

    pub fn create_example()->Result<(), Box<dyn Error>>
    {
        let mut example=Manifest{
            title:"Rotation Description Example".to_string(),
            descriptions:Vec::new(),
            baselines:Vec::new()
        };

        example.descriptions.push(
            RotationDescription { 
                rotation_name: "Rotation A".to_string(),
                responsibilities: vec![
                    RotationResponsibility{
                        site:"Site 1/Site 2",
                        subspecialty:"Subspecialty 1/Subspecialty 2",
                        context:"Context 1/Context 2",
                        modality:"Modality 1/Modality 2",
                        time_period:StringTypes::Array(vec!["8:00-12:00".to_string(), "13:00-17:00".to_string()]),
                        day:"every business day".to_string()
                    },
                    RotationResponsibility{
                        site:StringTypes::Array(vec!["Site A".to_string(),"Site B".to_string()]),
                        subspecialty:StringTypes::Array(vec!["Specialty A".to_string(),"Specialty B".to_string()]),
                        context:StringTypes::Array(vec!["Context A".to_string(),"Context B".to_string()]),
                        modality:StringTypes::Array(vec!["Modality A".to_string(),"Modality B".to_string()]),
                        time_period:StringTypes::Array(vec!["8:00-12:00".to_string(), "13:00-17:00".to_string()]),
                        day:"every business day".to_string()
                    }
                ]
            }
        );

        example.baselines.push(
            RotationBaseline { 
                rotation_name: "Rotation A".to_string()
            }
        );

        let writer = fs::File::create(EXAMPLE_ROTATION_DESCRIPTIONS)?;
        serde_yaml::to_writer(writer, &example);
        //serde_json::to_writer_pretty(writer, &example);

        Ok(())
    }
}