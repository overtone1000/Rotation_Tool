use std::fmt;

use chrono::NaiveTime;

use super::rotation_error::RotationManifestParseError;

pub const PREVIOUS_BUSINESS_DAY:&str="PBD";
pub const CURRENT_DAY:&str="CD";
pub const PREVIOUS_DAY:&str="PD";

#[derive(Debug, PartialEq)]
pub enum RelativeTime
{
    PreviousBusinessDay(NaiveTime),
    PreviousDay(NaiveTime),
    CurrentDay(NaiveTime),
}

const delimiter:&str=" ";

pub fn parse_relative_time(strval:&str)->Result<RelativeTime,RotationManifestParseError>
{
    let err=RotationManifestParseError::generate(0,format!("Malformed relative time {}",strval));

    let spl=strval.split(delimiter);
    let mut members = Vec::new();
    for item in spl
    {
        members.push(item);
    }

    if members.len()!=2 {return err;}

    let time = match NaiveTime::parse_from_str(members.get(0).expect("Checked"), "%H:%M")
    {
        Ok(x)=>x,
        Err(e)=>{return err;}
    };

    match *members.get(1).expect("Checked")
    {
        PREVIOUS_BUSINESS_DAY=>{Ok(RelativeTime::PreviousBusinessDay(time))},
        CURRENT_DAY=>{Ok(RelativeTime::PreviousBusinessDay(time))},
        PREVIOUS_DAY=>{Ok(RelativeTime::PreviousBusinessDay(time))},
        _=>{err}
    }
}

impl RelativeTime
{
    pub fn to_string(&self)->String{

        fn cc(time:&NaiveTime,modifier:&str)->String{
            time.to_string() + delimiter + PREVIOUS_BUSINESS_DAY
        }

        match self
        {
            RelativeTime::PreviousBusinessDay(time)=>cc(time,PREVIOUS_BUSINESS_DAY),
            RelativeTime::CurrentDay(time)=>cc(time,CURRENT_DAY),
            RelativeTime::PreviousDay(time)=>cc(time,PREVIOUS_DAY)
        }
    }
}