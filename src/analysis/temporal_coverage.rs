use std::fmt::Display;

use crate::rotations::time_modifiers::{TimeSinceMidnight, this_midnight, next_midnight};

use super::coverage_tree::{CoverageAndWorkDay, WorkCollector, AnalysisDatum};

pub fn weekday_plus(base_weekday:chrono::Weekday, delta:i64)->chrono::Weekday{
    let mut retval = base_weekday;
    if delta>0
    {
        for _ in 0..delta
        {
            retval=retval.succ();
        }
    }
    else if delta<0
    {
        for _ in delta..0
        {
            retval=retval.pred();
        }
    }
    retval
}

#[derive(Debug,PartialEq,Clone)]
pub struct TemporalCoverageUnit
{
    pub start:TimeSinceMidnight,
    pub end:TimeSinceMidnight,
    rotation:String,
    rotation_day:chrono::Weekday
}

impl Eq for TemporalCoverageUnit{}

impl TemporalCoverageUnit
{
    pub fn create(start:TimeSinceMidnight,end:TimeSinceMidnight,rotation:String,day:chrono::Weekday)->TemporalCoverageUnit
    {
        TemporalCoverageUnit
        {
            start:start,
            end:end,
            rotation:rotation,
            rotation_day:day
            //weekday_offset:offset //This is limited to one 24 hour period inclusive on each end.
        }
    }

    pub fn end_overlaps_other(&self, other:&TemporalCoverageUnit)->bool
    {
        //self.weekday_offset>other.weekday_offset || 
        //(self.weekday_offset==other.weekday_offset && self.end>other.start)
        self.end > other.start
    }

    pub fn gap_between_end_and_other(&self, other:&TemporalCoverageUnit)->bool
    {
        /*
        !(
            (self.weekday_offset==other.weekday_offset-1 && self.end==next_midnight && other.start==this_midnight) ||  //checks next_midnight/this_midnight contiguous
            (other.weekday_offset==self.weekday_offset-1 && self.end==this_midnight && other.start==next_midnight)     //checks this_midnight/next_midnight contiguous; it's an odd case that would require a zero-length coverage unit, but just check it      
        )
        &&
        (
            self.weekday_offset<other.weekday_offset ||
            (self.end<other.start)
        )
        */
        self.end<other.start
    }

    pub fn ends_after_other(&self, other:&TemporalCoverageUnit)->bool
    {
        //self.weekday_offset>other.weekday_offset ||
        self.end>other.end
    }

    pub fn starts_after_this_midnight(&self)->bool
    {
        /*
        self.weekday_offset>0 ||
        (self.weekday_offset==0 && 
            self.start>this_midnight)
        */
        self.start>this_midnight
    }

    pub fn ends_before_next_midnight(&self)->bool
    {
        /*
        self.weekday_offset<0 ||
        (self.weekday_offset==0 && 
            self.end<next_midnight)
        */
        self.end<next_midnight
    }

    /*
    fn get_shift_weekday(&self, base_weekday:chrono::Weekday)->chrono::Weekday
    {
        weekday_plus(base_weekday,-self.weekday_offset) //This back calculates the shift's weekday from the coverage info
    }
    */

    pub fn get_overlap_desc(farthest_unit:&TemporalCoverageUnit, cu:&TemporalCoverageUnit)->String{
        farthest_unit.to_string() + " goes to " + farthest_unit.end.to_string().as_str() + " and " + cu.to_string().as_str() + " starts at " + cu.start.to_string().as_str()
    }

    pub fn get_rotation(&self)->String{self.rotation.to_string()}
    pub fn get_day(&self)->chrono::Weekday{self.rotation_day}


    pub fn to_string(&self)->String
    {
        format!("{} ({})",self.rotation, self.rotation_day)
    }
}

impl PartialOrd for TemporalCoverageUnit
{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
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

impl Ord for TemporalCoverageUnit
{
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.partial_cmp(other)
        {
            Some(x)=>x,
            None=>core::cmp::Ordering::Equal
        }
    }
}

impl WorkCollector for TemporalCoverageUnit
{
    fn collect_work(&self, workday:&CoverageAndWorkDay)->AnalysisDatum
    {
        workday.get_work_in_timespan(self.start, self.end)
    }
}