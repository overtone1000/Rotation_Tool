
use serde::Serialize;

use crate::
    rotations::time_modifiers::{TimeSinceMidnight, NEXT_MIDNIGHT, THIS_MIDNIGHT}
;


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

#[derive(Debug, PartialEq, Clone, Serialize)]
pub struct TemporalCoverageUnit {
    pub start: TimeSinceMidnight,
    pub end: TimeSinceMidnight,
    rotation: String,
    //rotation_day: SerializeableWeekday
    work_to_rotation_day_offset:i64 //Day offset between work and rotation (relative to work, so + means work is before rotation, should generally be positive)
}

impl Eq for TemporalCoverageUnit {}

pub fn weekday_for_javascript(weekday: &chrono::Weekday) -> u32 {
    weekday.num_days_from_sunday() //This is how javascript represents weekdays
}

impl TemporalCoverageUnit {
    pub fn create(
        start: TimeSinceMidnight,
        end: TimeSinceMidnight,
        rotation: String,
        work_to_rotation_day_offset:i64,
    ) -> TemporalCoverageUnit {
        TemporalCoverageUnit {
            start,
            end,
            rotation,
            work_to_rotation_day_offset
            ,
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

    pub fn get_offset(&self) -> i64 {
        self.work_to_rotation_day_offset
    }


    pub fn to_string(&self) -> String {
        format!("{} (offset {} days)", self.rotation, self.work_to_rotation_day_offset)
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