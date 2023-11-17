use std::fmt;

use chrono::NaiveTime;
use serde::{Serialize, de::{Visitor, self}, Deserialize};

use super::{time_modifiers::{RelativeTime, parse_relative_time}, rotation_error::RotationManifestParseError};

#[derive(Debug, PartialEq)]
pub struct Timespan
{
    pub start:RelativeTime,
    pub end:RelativeTime
}

impl Serialize for Timespan
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer {
        serializer.serialize_str(&(
            self.start.to_string()
            +delimiter+
            &self.end.to_string()))
    }
}

const delimiter:&str="-";

pub fn parse_time_span(strval:&str)->Result<Timespan,RotationManifestParseError>
{
    let err=RotationManifestParseError::generate(0,format!("Malformed relative time {}",strval));

    let spl=strval.split(delimiter);
    let mut members = Vec::new();
    for item in spl
    {
        members.push(item);
    }

    if members.len()!=2 {return err;}

    let start=match parse_relative_time(members.get(0).expect("Checked"))
    {
        Ok(x)=>x,
        Err(e)=>{return Err(e);}
    };
    let end = match parse_relative_time(members.get(1).expect("Checked"))
    {
        Ok(x)=>x,
        Err(e)=>{return Err(e);}
    };

    Ok(
        Timespan { start: start, end: end }
    )
}

struct TimespanVisitor;
impl<'de> Visitor<'de> for TimespanVisitor {
    type Value = Timespan;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("A timespan in the format 'HH:mm Mod-HH:mm Mod' where 'Mod' is a relative time indicator 'PBD' for previous business day, 'CD' for current day, or 'PD' for previous day. Times are in 24-hours.")
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: de::Error
    {
        match parse_time_span(value)
        {
            Ok(x)=>Ok(x),
            Err(e)=>Err(de::Error::custom(e))
        }
    }
}

impl<'de> Deserialize<'de> for Timespan
{
    fn deserialize<D>(deserializer:D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de> {
        Ok(deserializer.deserialize_str(TimespanVisitor)?)
    }
}