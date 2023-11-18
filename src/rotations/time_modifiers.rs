use std::fmt;

use chrono::NaiveTime;

use super::{rotation_error::RotationManifestParseError, timespan::midnight};

pub const PREVIOUS_BUSINESS_DAY:&str="PBD";
pub const CURRENT_DAY:&str="CD";
pub const PREVIOUS_DAY:&str="PD";

#[derive(Debug, PartialEq)]
pub enum RelativeTime
{
    PreviousBusinessDay(NaiveTime,bool),
    PreviousDay(NaiveTime,bool),
    CurrentDay(NaiveTime,bool),
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
    let mut nextday=false;

    let time = match *members.get(0).expect("Checked")
    {
        nextmidnightstr=>{
            nextday=true;
            midnight()
        },
        x=>{
            match NaiveTime::parse_from_str(x, "%H:%M")
            {
                Ok(x)=>x,
                Err(e)=>{return err;}
            }
        }

    };

    match *members.get(1).expect("Checked")
    {
        PREVIOUS_BUSINESS_DAY=>{Ok(RelativeTime::PreviousBusinessDay(time,nextday))},
        PREVIOUS_DAY=>{Ok(RelativeTime::PreviousDay(time,nextday))},
        CURRENT_DAY=>{Ok(RelativeTime::CurrentDay(time,nextday))},
        _=>{err}
    }
}

const nextmidnightstr:&str="24:00";

impl RelativeTime
{
    pub fn to_string(&self)->String{

        fn cc(time:&NaiveTime,modifier:&str, nextmidnight:&bool)->String{
            let timestr = match nextmidnight{
                true=>nextmidnight.to_string(),
                false=>time.to_string()
            };
            timestr + delimiter + PREVIOUS_BUSINESS_DAY
        }

        match self
        {
            RelativeTime::PreviousBusinessDay(time, nextmidnight)=>cc(time,PREVIOUS_BUSINESS_DAY, nextmidnight),
            RelativeTime::CurrentDay(time, nextmidnight)=>cc(time,CURRENT_DAY, nextmidnight),
            RelativeTime::PreviousDay(time, nextmidnight)=>cc(time,PREVIOUS_DAY, nextmidnight)
        }
    }
}