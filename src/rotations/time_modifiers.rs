use std::fmt;

//use chrono::NaiveTime;

use super::{rotation_error::RotationManifestParseError};

const PREVIOUS_BUSINESS_DAY:&str="PBD";
//const DAY_AFTER_PREVIOUS_BUSINESS_DAY:&str="PBD+1";
const PREVIOUS_DAY:&str="PD";
const CURRENT_DAY:&str="CD";
const NEXT_DAY:&str="ND";

#[derive(PartialEq, Eq, PartialOrd, Ord, Default, Clone, Copy)]
pub struct TimeSinceMidnight
{
    minutes:u64,
}

pub const this_midnight:TimeSinceMidnight = TimeSinceMidnight{minutes:0};
pub const next_midnight:TimeSinceMidnight = TimeSinceMidnight{minutes:24*60};

impl TimeSinceMidnight
{
    pub fn hours(&self)->u64{
        self.minutes/60
    }

    pub fn minutes(&self)->u64{
        self.minutes%60
    }

    pub fn parse_from_str(str:&str)->Result<TimeSinceMidnight,()>{
        let split:Vec<&str> = str.split(":").collect();
        if split.len()!=2 {return Err(());}

        let hrs:u64 = match split.get(0).expect("Checked").parse()
        {
            Ok(x) => x,
            Err(_) => {return Err(());},
        };

        let min:u64 = match split.get(1).expect("Checked").parse()
        {
            Ok(x) => x,
            Err(_) => {return Err(());},
        };

        let retval = TimeSinceMidnight { 
            minutes: hrs*60+min
        };

        if retval.is_valid()
        {
            Ok(retval)
        }
        else {
            Err(())
        }
    }

    pub fn is_valid(&self)->bool{
        self<=&next_midnight
    }
}

impl fmt::Debug for TimeSinceMidnight
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let str = self.to_string();
        f.debug_struct("TimeSinceMidnight").field("time", &str).finish()
    }
}

impl fmt::Display for TimeSinceMidnight
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let str=self.hours().to_string()+":"+&self.minutes().to_string();
        write!(f,"{}",str)
    }
}

#[derive(Debug, PartialEq)]
pub enum RelativeTime
{
    PreviousBusinessDay(TimeSinceMidnight),
    //DayAfterPreviousBusinessDay(NaiveTime),
    PreviousDay(TimeSinceMidnight),
    CurrentDay(TimeSinceMidnight),
    NextDay(TimeSinceMidnight)
}

const delimiter:&str=" ";

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

impl RelativeTime
{
    pub fn get_modifier(&self)->&str{
        match self 
        {
            Self::PreviousBusinessDay(_)=>PREVIOUS_BUSINESS_DAY,
            //Self::DayAfterPreviousBusinessDay(_)=>DAY_AFTER_PREVIOUS_BUSINESS_DAY,
            Self::PreviousDay(_)=>PREVIOUS_DAY,
            Self::CurrentDay(_)=>CURRENT_DAY,
            Self::NextDay(_)=>NEXT_DAY
        }
    }

    pub fn get_time(&self)->&TimeSinceMidnight
    {
        match self
        {
            RelativeTime::PreviousBusinessDay(x) => x,
            //RelativeTime::DayAfterPreviousBusinessDay(x) => x,
            RelativeTime::PreviousDay(x) => x,
            RelativeTime::CurrentDay(x) => x,
            RelativeTime::NextDay(x) => x,
        }
    }

    pub fn get_day(&self,day:chrono::Weekday)->chrono::Weekday{
        match self{
            RelativeTime::PreviousBusinessDay(x) => {
                get_previous_business_day(day)
            },
            //RelativeTime::DayAfterPreviousBusinessDay(_) => {
            //    get_previous_business_day(day).succ()
            //},
            RelativeTime::PreviousDay(x) => {
                day.pred()
            },
            RelativeTime::CurrentDay(x) => {
                day
            },
            RelativeTime::NextDay(x) => {
                day.succ()
            },
        }
    }
}

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

    let time = match *members.get(0).expect("Checked")
    {
        x=>{
            match TimeSinceMidnight::parse_from_str(x)
            {
                Ok(x)=>x,
                Err(e)=>{
                    return err;
                }
            }
        }

    };

    match *members.get(1).expect("Checked")
    {
        PREVIOUS_BUSINESS_DAY=>{Ok(RelativeTime::PreviousBusinessDay(time))},
        //DAY_AFTER_PREVIOUS_BUSINESS_DAY=>{Ok(RelativeTime::DayAfterPreviousBusinessDay(time))},
        PREVIOUS_DAY=>{Ok(RelativeTime::PreviousDay(time))},
        CURRENT_DAY=>{Ok(RelativeTime::CurrentDay(time))},
        NEXT_DAY=>{Ok(RelativeTime::NextDay(time))},
        _=>{
            err
        }
    }
}

impl RelativeTime
{
    pub fn to_string(&self)->String{
        let time = self.get_time();

        let timestr = time.to_string();
        
        timestr + delimiter + self.get_modifier()
    }
}