use std::fmt::Display;

use crate::rotations::time_modifiers::TimeSinceMidnight;

pub fn weekday_plus(base_weekday:chrono::Weekday, delta:i64)->chrono::Weekday{
    let mut retval = base_weekday;
    if delta>1
    {
        for _ in 0..delta
        {
            retval=retval.succ();
        }
    }
    else if delta<1
    {
        for _ in 0..delta
        {
            retval=retval.pred();
        }
    }
    retval
}

#[derive(Debug,PartialEq,Eq)]
pub struct CoverageUnit
{
    pub start:TimeSinceMidnight, //Make first so it's sorted on start time first!
    pub end:TimeSinceMidnight, //Make second so it's sorted on end time next!
    rotation:String,
    weekday_offset:i64
}

impl CoverageUnit
{
    pub fn create(start:TimeSinceMidnight,end:TimeSinceMidnight,rotation:String,offset:i64)->CoverageUnit
    {
        CoverageUnit
        {
            start:start,
            end:end,
            rotation:rotation,
            weekday_offset:offset
        }
    }

    fn get_shift_weekday(&self, base_weekday:chrono::Weekday)->chrono::Weekday
    {
        weekday_plus(base_weekday,-self.weekday_offset) //This back calculates the shift's weekday from the coverage info
    }
    pub fn to_string(&self, base_weekday:chrono::Weekday)->String
    {
        format!("{} ({})",self.rotation,self.get_shift_weekday(base_weekday))
    }
}

impl PartialOrd for CoverageUnit
{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.weekday_offset.partial_cmp(&other.weekday_offset){
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        match self.start.partial_cmp(&other.start) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        match self.end.partial_cmp(&other.end) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        self.rotation.partial_cmp(&other.rotation)
    }
}

impl Ord for CoverageUnit
{
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.partial_cmp(other)
        {
            Some(x)=>x,
            None=>core::cmp::Ordering::Equal
        }
    }
}