use std::fmt;

use chrono::NaiveTime;
use serde::{Serialize, de::{Visitor, self}, Deserialize};

#[derive(Debug, PartialEq)]
pub struct Timespan
{
    start:NaiveTime,
    end:NaiveTime
}

impl Serialize for Timespan
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer {
        serializer.serialize_str(&(self.start.to_string()+"-"+&self.end.to_string()))
    }
}

struct NaiveTimeVisitor;
impl<'de> Visitor<'de> for NaiveTimeVisitor {
    type Value = Timespan;

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
            Timespan{
                start:*vec.get(0).expect("Unexpected vector malformation"),
                end:*vec.get(1).expect("Unexpected vector malformation")
            }
        )
    }
}

impl<'de> Deserialize<'de> for Timespan
{
    fn deserialize<D>(deserializer:D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de> {
        Ok(deserializer.deserialize_str(NaiveTimeVisitor)?)
    }
}