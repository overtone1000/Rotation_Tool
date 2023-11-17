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

impl StringTypes
{
    pub fn new_slash_separated_string_vec(val:&str)->StringTypes
    {
        StringTypes::SlashSeparatedStringVec(SlashSeparatedStringVec::new(val))
    }

    pub fn to_vec(&self)->Vec<String>
    {
        match self
        {
            StringTypes::SlashSeparatedStringVec(x) => x.values,
            StringTypes::Array(x) => *x,
        }
    }
}

const delimiter:&str="/";

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
            let mut str:Option<String>=None;
            for value in &self.values
            {
                match str
                {
                    None=>{str=Some(value.to_string())},
                    Some(x)=>{str=Some(x+&delimiter+value)}
                }
            }
            match str
            {
                None=>{serializer.serialize_none()},
                Some(x)=>{serializer.serialize_str(&x)}
            }
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