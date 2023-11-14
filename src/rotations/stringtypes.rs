use std::fmt;

use chrono::NaiveTime;
use serde::{Serialize, Deserialize, de::{Visitor, self}, Serializer};

use super::timespan::Timespan;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum StringTypes
{
    SlashSeparatedStringVec(SlashSeparatedStringVec),
    Array(Vec<String>)
}

const delimiter:String="/".to_string();

#[derive(Debug, PartialEq)]
pub struct SlashSeparatedStringVec
{
    values:Vec<String>
}

impl SlashSeparatedStringVec
{
    pub fn new(val:&str)->SlashSeparatedStringVec{
        let mut vec:Vec<String>=Vec::new();
        let values=val.split(&delimiter);
        for value in values
        {
            vec.push(value.to_string());
        }
        SlashSeparatedStringVec{
            values:vec
        }
    }
}

impl Serialize for SlashSeparatedStringVec
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer {
            let mut retval:Result<S::Ok, S::Error>;
            for value in &self.values
            {
                let cc = value.to_string()+&delimiter;
                retval=serializer.serialize_str(&cc);
                if retval.is_err() {return retval;}
            }
           retval
    }
}

struct StringStringVisitor;
impl<'de> Visitor<'de> for StringStringVisitor {
    type Value = SlashSeparatedStringVec;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        let str="A set of strings separated by ".to_string() + &delimiter;
        formatter.write_str(&str)
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {        
        Ok(
            SlashSeparatedStringVec::new(value)
        )
    }
}

impl<'de> Deserialize<'de> for SlashSeparatedStringVec
{
    fn deserialize<D>(deserializer:D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de> {
        Ok(deserializer.deserialize_str(StringStringVisitor)?)
    }
}