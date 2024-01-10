use std::collections::HashSet;

use serde::{Deserialize, Serialize};

use super::{responsibility::{RotationResponsibility}, rotation_error::RotationManifestParseError, timespan::Timespan, stringtypes::StringTypes, time_modifiers::RelativeTime};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct RotationHours {
    hours:Timespan,
    days:StringTypes
}

impl RotationHours {
    pub fn new(start:RelativeTime,stop:RelativeTime,days:StringTypes)->RotationHours{
        RotationHours{
            hours:Timespan { start: start, stop: stop },
            days:days
        }
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RotationDescription {
    pub(crate) rotation: String,
    pub(crate) location: String,
    pub(crate) hours: Option<Vec<RotationHours>>,
    pub(crate) breaktime: Option<(Timespan,Option<String>)>,
    pub(crate) responsibilities: Responsibilities,
    pub(crate) comments: Option<HashSet<String>>,
}

impl PartialOrd for RotationDescription
{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.responsibilities.partial_cmp(&other.responsibilities) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        match self.rotation.partial_cmp(&other.rotation) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        self.location.partial_cmp(&other.location)
    }
}

#[derive(Debug, PartialEq, PartialOrd)]
pub struct Responsibilities
{
    value: Option<Vec<RotationResponsibility>>
}

impl Responsibilities
{
    pub fn from_vec(vec:Vec<RotationResponsibility>)->Responsibilities
    {
        Responsibilities { value: Some(vec) }
    }
}

pub trait WrappedSortable<T>
where T:Ord
{
    fn get(&self)->&Option<Vec<T>>;
    fn fromval(val:Option<Vec<T>>)->Self;
}

impl WrappedSortable<RotationResponsibility> for Responsibilities
{
    fn get(&self)->&Option<Vec<RotationResponsibility>>{
        &self.value
    }
    fn fromval(mut val:Option<Vec<RotationResponsibility>>)->Responsibilities{
        match val
        {
            Some(mut x)=>{
                x.sort();
                val=Some(x);
            },
            None => {();},
        }
        
        Responsibilities{
            value:val
        }
    }
}

impl <'de> Deserialize<'de> for Responsibilities
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de> {
        let mut val = Option::<Vec::<RotationResponsibility>>::deserialize(deserializer)?;
        match &mut val
        {
            Some(x) => {x.sort();},
            None => {},
        }
        Ok(Responsibilities::fromval(val))
    }
}
impl Serialize for Responsibilities
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer {
        match &self.value
        {
            Some(x) => {
                let mut newvec=x.clone();
                newvec.serialize(serializer)
            },
            None => {
                serializer.serialize_none()
            },
        }
    }
}