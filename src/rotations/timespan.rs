use std::{fmt::{self, Display}, f32::consts::E};

use chrono::NaiveTime;
use serde::{Serialize, de::{Visitor, self}, Deserialize};

use super::{time_modifiers::{RelativeTime, parse_relative_time}, rotation_error::RotationManifestParseError};

pub fn midnight()->NaiveTime{NaiveTime::from_hms_opt(0,0,0).expect("Midnight")}

#[derive(Debug, PartialEq)]
pub struct Timespan
{
    pub start:RelativeTime,
    pub stop:RelativeTime
}

impl Display for Timespan
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}-{}", self.start.to_string(), self.stop.to_string())
    }
}

impl Serialize for Timespan
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer {
        serializer.serialize_str(&(
            self.start.to_string()
            +delimiter+
            &self.stop.to_string()))
    }
}

const delimiter:&str="-";


fn get_previous_business_day(day:chrono::Weekday)->chrono::Weekday{
    match day
    {
        chrono::Weekday::Mon => chrono::Weekday::Fri,
        chrono::Weekday::Tue => chrono::Weekday::Mon,
        chrono::Weekday::Wed => chrono::Weekday::Tue,
        chrono::Weekday::Thu => chrono::Weekday::Wed,
        chrono::Weekday::Fri => chrono::Weekday::Thu,
        chrono::Weekday::Sat => chrono::Weekday::Fri,
        chrono::Weekday::Sun => chrono::Weekday::Fri,
    }
}

fn get_intervening_days(start:chrono::Weekday, end:chrono::Weekday)->Vec<chrono::Weekday>{
    let mut retval:Vec<chrono::Weekday>=Vec::new();

    if start==end {return retval;}

    let mut next=start.succ();
    while next!=end
    {
        retval.push(next);
        next=next.succ();
    }

    retval
}

impl Timespan
{
    fn is_valid(&self)->bool
    {
        match self.start
        {
            RelativeTime::PreviousBusinessDay(start) => 
            {
                match self.stop{
                    RelativeTime::PreviousBusinessDay(end) => start<end,
                    RelativeTime::PreviousDay(end) => start<end,
                    RelativeTime::CurrentDay(_) => true,
                }
            },
            RelativeTime::PreviousDay(start) => 
            {
                match self.stop{
                    RelativeTime::PreviousBusinessDay(end) => start<end,
                    RelativeTime::PreviousDay(end) => start<end,
                    RelativeTime::CurrentDay(_) => true,
                }
            },
            RelativeTime::CurrentDay(start) => 
            {
                match self.stop{
                    RelativeTime::PreviousBusinessDay(_) => false,
                    RelativeTime::PreviousDay(_) => false,
                    RelativeTime::CurrentDay(end) => start<end,
                }
            },
        }
    }
    pub fn instantiate_periods(&self, day:chrono::Weekday)->Vec<(chrono::Weekday,NaiveTime,NaiveTime)>
    {
        let mut retval :Vec<(chrono::Weekday,NaiveTime,NaiveTime)>=Vec::new();

        let startday:chrono::Weekday;
        let stopday:chrono::Weekday;
        let start:NaiveTime;
        let stop:NaiveTime;

        match self.start
        {
            RelativeTime::PreviousBusinessDay(x) => {
                startday=day;
                start=x;
            },
            RelativeTime::PreviousDay(x) => {
                startday=day.pred();
                start=x;
            },
            RelativeTime::CurrentDay(x) => {
                startday=get_previous_business_day(day);
                start=x;
            },
        };

        match self.stop
        {
            RelativeTime::PreviousBusinessDay(x) => {
                stopday=day;
                stop=x;
            },
            RelativeTime::PreviousDay(x) => {
                stopday=day.pred();
                stop=x;
            },
            RelativeTime::CurrentDay(x) => {
                stopday=get_previous_business_day(day);
                stop=x;
            },
        };

        if startday==stopday
        {
            retval.push((startday,start,stop))
        }
        else
        {
            retval.push((startday,start,midnight()));
            for day in get_intervening_days(startday,stopday)
            {
                retval.push((day,midnight(),midnight()));
            }
            retval.push((stopday,midnight(),stop));    
        }

        retval
    }
}

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

    let ts =Timespan { start: start, stop: end };
    if ts.is_valid()
    {
        Ok(ts)
    }
    else {
        RotationManifestParseError::generate(0, format!("Invalid timespan {}",ts))
    }
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