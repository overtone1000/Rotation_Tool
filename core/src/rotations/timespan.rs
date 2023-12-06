use std::fmt::{self, Display};

use chrono::{Duration, NaiveTime};
use serde::{
    de::{self, Visitor},
    Deserialize, Serialize,
};

use super::{
    rotation_error::RotationManifestParseError,
    time_modifiers::{
        next_midnight, parse_relative_time, this_midnight, RelativeTime, TimeSinceMidnight,
    },
};

#[derive(Debug, PartialEq)]
pub struct Timespan {
    pub start: RelativeTime,
    pub stop: RelativeTime,
}

impl Display for Timespan {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}{}{}",
            self.start.to_string(),
            DELIMITER,
            self.stop.to_string()
        )
    }
}

impl Serialize for Timespan {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&(self.start.to_string() + DELIMITER + &self.stop.to_string()))
    }
}

const DELIMITER: &str = "-";

fn get_intervening_days(start: chrono::Weekday, end: chrono::Weekday) -> Vec<chrono::Weekday> {
    let mut retval: Vec<chrono::Weekday> = Vec::new();

    if start == end {
        return retval;
    }

    let mut next = start.succ();
    while next != end {
        retval.push(next);
        next = next.succ();
    }

    retval
}

impl Timespan {
    pub fn instantiate_periods(
        &self,
        day: chrono::Weekday,
    ) -> Vec<(i64, TimeSinceMidnight, TimeSinceMidnight)> {
        let mut retval: Vec<(i64, TimeSinceMidnight, TimeSinceMidnight)> = Vec::new();

        let startday = self.start.get_day_offset(day);
        let stopday = self.stop.get_day_offset(day);
        let start = *self.start.get_time();
        let stop = *self.stop.get_time();

        if startday == stopday {
            retval.push((startday, start, stop))
        } else {
            retval.push((startday, start, next_midnight));
            for day in startday + 1..stopday {
                retval.push((day, this_midnight, next_midnight));
            }
            retval.push((stopday, this_midnight, stop));
        }

        retval
    }
}

pub fn parse_time_span(strval: &str) -> Result<Timespan, RotationManifestParseError> {
    let err =
        RotationManifestParseError::generate(0, format!("Malformed relative time {}", strval));

    let spl = strval.split(DELIMITER);
    let mut members = Vec::new();
    for item in spl {
        members.push(item);
    }

    if members.len() != 2 {
        return err;
    }

    let start = match parse_relative_time(members.get(0).expect("Checked")) {
        Ok(x) => x,
        Err(e) => {
            return Err(e);
        }
    };
    let end = match parse_relative_time(members.get(1).expect("Checked")) {
        Ok(x) => x,
        Err(e) => {
            return Err(e);
        }
    };

    let ts = Timespan {
        start: start,
        stop: end,
    };
    Ok(ts)
}

struct TimespanVisitor;
impl<'de> Visitor<'de> for TimespanVisitor {
    type Value = Timespan;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("A timespan in the format 'HH:mm Mod-HH:mm Mod' where 'Mod' is a relative time indicator 'PBD' for previous business day, 'CD' for current day, or 'PD' for previous day. Times are in 24-hours.")
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        match parse_time_span(value) {
            Ok(x) => Ok(x),
            Err(e) => Err(de::Error::custom(e)),
        }
    }
}

impl<'de> Deserialize<'de> for Timespan {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(deserializer.deserialize_str(TimespanVisitor)?)
    }
}
