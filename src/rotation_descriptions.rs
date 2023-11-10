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
#[serde(untagged)]
enum StringOrStringVec
{
    Singleton(String),
    Array(Vec<String>)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct RotationDescriptionsDocument
{
    title:String,
    descriptions:Vec<RotationDescription>,
    baselines:Vec<RotationBaseline>
}

impl RotationDescriptionsDocument
{
    pub fn parse(filename:&String)->Result<RotationDescriptionsDocument, Box<dyn Error>>
    {
        let rdr = fs::File::open(filename)?;
        let retval:RotationDescriptionsDocument=serde_yaml::from_reader(rdr)?;

        Ok(retval)
    }

    pub fn create_example()->Result<(), Box<dyn Error>>
    {
        let mut example=RotationDescriptionsDocument{
            title:"Rotation Description Example".to_string(),
            descriptions:Vec::new(),
            baselines:Vec::new()
        };

        example.descriptions.push(
            RotationDescription { 
                rotation_name: "Rotation A".to_string(),
                responsibilities: vec![
                    RotationResponsibility{
                        site:StringOrStringVec::Singleton("Site 1".to_string()),
                        subspecialty:StringOrStringVec::Singleton("Specialty 1".to_string()),
                        context:StringOrStringVec::Singleton("Context 1".to_string()),
                        modality:StringOrStringVec::Singleton("Modality 1".to_string()),
                        time_period:StringOrStringVec::Singleton("10:00-11:00".to_string()),
                        day:"previous business day".to_string()
                    },
                    RotationResponsibility{
                        site:StringOrStringVec::Array(vec!["Site A".to_string(),"Site B".to_string()]),
                        subspecialty:StringOrStringVec::Array(vec!["Specialty A".to_string(),"Specialty B".to_string()]),
                        context:StringOrStringVec::Array(vec!["Context A".to_string(),"Context B".to_string()]),
                        modality:StringOrStringVec::Array(vec!["Modality A".to_string(),"Modality B".to_string()]),
                        time_period:StringOrStringVec::Array(vec!["8:00-12:00".to_string(), "13:00-17:00".to_string()]),
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

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct RotationDescription
{
    rotation_name:String,
    responsibilities:Vec<RotationResponsibility>
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct RotationBaseline
{
    rotation_name:String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct RotationResponsibility
{
    site:StringOrStringVec,
    subspecialty:StringOrStringVec,
    context:StringOrStringVec,
    modality:StringOrStringVec,
    time_period:StringOrStringVec,
    day:String
}

#[derive(Debug, PartialEq)]
pub struct TimespanString
{
    start:NaiveTime,
    end:NaiveTime
}

impl Serialize for TimespanString
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer {
        serializer.serialize_str(&(self.start.to_string()+"-"+&self.end.to_string()))
    }
}

struct NaiveTimeVisitor;
impl<'de> Visitor<'de> for NaiveTimeVisitor {
    type Value = TimespanString;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("A time in the format HH:mm")
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        let mut vec:Vec<NaiveTime>=Vec::new();
        let values=value.split("-");
        for value in values
        {
            let time = match NaiveTime::parse_from_str(value, "%H:%M")
            {
                Ok(x)=>x,
                Err(e)=>{return Err(de::Error::custom(e.to_string()));}
            };
            vec.push(time);
        }
        if vec.len()!=2 {return Err(de::Error::custom("Incorrect number of times provided."))}
        
        
        Ok(
            TimespanString{
                start:*vec.get(0).expect("Unexpected vector malformation"),
                end:*vec.get(1).expect("Unexpected vector malformation")
            }
        )
    }
}

impl<'de> Deserialize<'de> for TimespanString
{
    fn deserialize<D>(deserializer:D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de> {
        Ok(deserializer.deserialize_str(NaiveTimeVisitor)?)
    }
}