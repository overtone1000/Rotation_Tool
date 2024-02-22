

use std::collections::HashMap;

use chrono::Datelike;
use serde::{Serialize, ser::SerializeStruct};

use crate::rotations::time_modifiers::{NEXT_MIDNIGHT, THIS_MIDNIGHT, TimeSinceMidnight};

use super::{coverage_tree::{CoverageAndWorkDay, WorkCollector}, analysis_datum::AnalysisDatum, fractional_coverage::SerializeableWeekday};

pub fn weekday_plus(base_weekday: chrono::Weekday, delta: i64) -> chrono::Weekday {
    let mut retval = base_weekday;
    if delta > 0 {
        for _ in 0..delta {
            retval = retval.succ();
        }
    } else if delta < 0 {
        for _ in delta..0 {
            retval = retval.pred();
        }
    }
    retval
}

#[derive(Debug, PartialEq, Clone,Serialize)]
pub struct TemporalCoverageUnit {
    pub start: TimeSinceMidnight,
    pub end: TimeSinceMidnight,
    rotation: String,
    rotation_day:SerializeableWeekday
}

impl Eq for TemporalCoverageUnit {}

pub fn weekday_for_javascript(weekday:&chrono::Weekday)->u32
{
    weekday.num_days_from_sunday()//This is how javascript represents weekdays
}

impl TemporalCoverageUnit {
    pub fn create(
        start: TimeSinceMidnight,
        end: TimeSinceMidnight,
        rotation: String,
        day: chrono::Weekday,
    ) -> TemporalCoverageUnit {
        TemporalCoverageUnit {
            start,
            end,
            rotation,
            rotation_day: SerializeableWeekday{day:day}, //weekday_offset:offset //This is limited to one 24 hour period inclusive on each end.
        }
    }

    pub fn end_overlaps_other(&self, other: &TemporalCoverageUnit) -> bool {
        //self.weekday_offset>other.weekday_offset ||
        //(self.weekday_offset==other.weekday_offset && self.end>other.start)
        self.end > other.start
    }

    pub fn gap_between_end_and_other(&self, other: &TemporalCoverageUnit) -> bool {
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
        self.end < other.start
    }

    pub fn ends_after_other(&self, other: &TemporalCoverageUnit) -> bool {
        //self.weekday_offset>other.weekday_offset ||
        self.end > other.end
    }

    pub fn starts_after_this_midnight(&self) -> bool {
        /*
        self.weekday_offset>0 ||
        (self.weekday_offset==0 &&
            self.start>this_midnight)
        */
        self.start > THIS_MIDNIGHT
    }

    pub fn ends_before_next_midnight(&self) -> bool {
        /*
        self.weekday_offset<0 ||
        (self.weekday_offset==0 &&
            self.end<next_midnight)
        */
        self.end < NEXT_MIDNIGHT
    }

    /*
    fn get_shift_weekday(&self, base_weekday:chrono::Weekday)->chrono::Weekday
    {
        weekday_plus(base_weekday,-self.weekday_offset) //This back calculates the shift's weekday from the coverage info
    }
    */

    pub fn get_overlap_desc(
        farthest_unit: &TemporalCoverageUnit,
        cu: &TemporalCoverageUnit,
    ) -> String {
        farthest_unit.to_string()
            + " goes to "
            + farthest_unit.end.to_string().as_str()
            + " and "
            + cu.to_string().as_str()
            + " starts at "
            + cu.start.to_string().as_str()
    }

    pub fn get_rotation(&self) -> String {
        self.rotation.to_string()
    }
    pub fn get_day(&self) -> chrono::Weekday {
        self.rotation_day.day
    }

    pub fn to_string(&self) -> String {
        format!("{} ({})", self.rotation, self.rotation_day.day)
    }
}

impl PartialOrd for TemporalCoverageUnit {
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

impl Ord for TemporalCoverageUnit {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.partial_cmp(other) {
            Some(x) => x,
            None => core::cmp::Ordering::Equal,
        }
    }
}

impl WorkCollector for TemporalCoverageUnit {
    fn collect_work(&self, workday: &CoverageAndWorkDay) -> AnalysisDatum {
        let mut retval:AnalysisDatum=AnalysisDatum::default();
        for work_unit in workday.get_work_in_timespan(self.start, self.end)
        {
            retval.add_workunit(work_unit);
        }
        retval
    }

    fn collect_work_bydate(&self, workday: &CoverageAndWorkDay) -> HashMap<chrono::prelude::NaiveDate,AnalysisDatum> {
        let mut retval: HashMap<chrono::prelude::NaiveDate,AnalysisDatum> = HashMap::new();

        for work_unit in workday.get_work_in_timespan(self.start, self.end) {            
            match retval.entry(work_unit.get_datetime().date())
            {
                std::collections::hash_map::Entry::Occupied(mut entry) => {
                    entry.get_mut().add_workunit(work_unit);
                },
                std::collections::hash_map::Entry::Vacant(empty) => {
                    let mut newdat=AnalysisDatum::default();
                    newdat.add_workunit(work_unit);
                    empty.insert(newdat);
                },
            };
        }

        retval
    }
}
