use std::{collections::HashSet, fmt};


use serde::{
    de::{self, Visitor},
    Deserialize, Serialize, Serializer, ser::SerializeSeq,
};



#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum StringTypes {
    All(AllType),
    SlashSeparatedStringHashSet(SlashSeparatedStringSet),
    Array(HashSet<String>),
}

impl StringTypes {
    pub fn new_slash_separated_string_vec(val: &str) -> StringTypes {
        StringTypes::SlashSeparatedStringHashSet(SlashSeparatedStringSet::new(val))
    }

    pub fn to_vec(&self, all_case: &[&str]) -> HashSet<String> {
        match self {
            StringTypes::All(_) => {
                let mut i: HashSet<String> = HashSet::new();
                for str in all_case {
                    i.insert(str.to_string());
                }
                i
            }
            StringTypes::SlashSeparatedStringHashSet(x) => x.values.to_owned(),
            StringTypes::Array(x) => x.to_owned(),
        }
    }

    pub fn validate(&self, allowed_members: &[&str]) -> Result<(), HashSet<String>> {
        let vec = match self {
            StringTypes::All(_) => {
                return Ok(());
            }
            StringTypes::SlashSeparatedStringHashSet(x) => x.values.to_owned(),
            StringTypes::Array(x) => x.to_owned(),
        };

        let mut invalids: HashSet<String> = HashSet::new();

        for str in vec {
            if !allowed_members.contains(&str.as_str()) {
                invalids.insert(str);
            }
        }

        if !invalids.is_empty() {
            Err(invalids)
        } else {
            Ok(())
        }
    }
}

const DELIMITER: &str = "/";

const ALL: &str = "All";

#[derive(Debug, PartialEq)]
pub struct AllType {}

impl Serialize for AllType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(ALL)
    }
}

struct AllTypeVisitor;
impl<'de> Visitor<'de> for AllTypeVisitor {
    type Value = AllType;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        let str = format!("Just the word {}", ALL);
        formatter.write_str(&str)
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        if value == ALL {
            Ok(AllType {})
        } else {
            Err(E::custom("Not an all"))
        }
    }
}

impl<'de> Deserialize<'de> for AllType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_str(AllTypeVisitor)
    }
}

#[derive(Debug, PartialEq)]
pub struct SlashSeparatedStringSet {
    values: HashSet<String>,
}

impl SlashSeparatedStringSet {
    pub fn new(val: &str) -> SlashSeparatedStringSet {
        let mut vec: HashSet<String> = HashSet::new();
        let values = val.split(&DELIMITER);
        for value in values {
            vec.insert(value.to_string());
        }
        SlashSeparatedStringSet { values: vec }
    }
}

impl Serialize for SlashSeparatedStringSet {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(self.values.len()))?;
        for value in &self.values {
            seq.serialize_element(value)?;
        }
        seq.end()
    }
}

struct SlashSeparateddStringVisitor;
impl<'de> Visitor<'de> for SlashSeparateddStringVisitor {
    type Value = SlashSeparatedStringSet;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        let str = "A set of strings separated by ".to_string() + &DELIMITER;
        formatter.write_str(&str)
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(SlashSeparatedStringSet::new(value))
    }
}

impl<'de> Deserialize<'de> for SlashSeparatedStringSet {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_str(SlashSeparateddStringVisitor)
    }
}
