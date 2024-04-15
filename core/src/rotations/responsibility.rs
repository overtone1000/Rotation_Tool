use serde::{Deserialize, Serialize};

use crate::globals;

use super::{
    description::WrappedSortable, rotation_error::RotationManifestParseError,
    stringtypes::StringTypes, timespan::Timespan,
};

pub fn check(t: &StringTypes, poss: &[&str], desc: &str, errors: &mut Vec<String>) {
    match t.validate(poss) {
        Err(e) => {
            for i in e {
                errors.push(format!(
                    "Invalid {} {}. Valid values are {:?}",
                    desc, i, poss
                ));
            }
        }
        _ => (),
    };
}

pub fn validate_days(days_to_check: &StringTypes, errors: &mut Vec<String>) {
    let mon = chrono::Weekday::Mon.to_string();
    let tue = chrono::Weekday::Tue.to_string();
    let wed = chrono::Weekday::Wed.to_string();
    let thu = chrono::Weekday::Thu.to_string();
    let fri = chrono::Weekday::Fri.to_string();
    let sat = chrono::Weekday::Sat.to_string();
    let sun = chrono::Weekday::Sun.to_string();

    let days = &[
        mon.as_str(),
        tue.as_str(),
        wed.as_str(),
        thu.as_str(),
        fri.as_str(),
        sat.as_str(),
        sun.as_str(),
    ];

    check(days_to_check, days, "weekday", errors);
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct RotationResponsibility {
    pub sites: StringTypes,
    pub exams: StringTypes,
    pub contexts: StringTypes,
    //pub modalities: StringTypes,
    pub days: StringTypes,
    pub weekly_fraction: Option<f64>,
    pub time_periods: TimePeriods,
}

impl RotationResponsibility {
    pub fn validate(&self) -> Result<(), Vec<String>> {
        let mut errors: Vec<String> = Vec::new();

        check(&self.sites, globals::FACILITIES, "site", &mut errors);
        check(
            &self.exams,
            globals::SUBSPECIALTIES,
            "subspecialty",
            &mut errors,
        );
        check(&self.contexts, globals::CONTEXTS, "context", &mut errors);
        //check(&self.modalities, globals::MODALITIES, "modality", &mut errors);
        validate_days(&self.days, &mut errors);

        if !errors.is_empty() {
            Err(errors)
        } else {
            Ok(())
        }
    }
}

impl PartialOrd for RotationResponsibility {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.weekly_fraction.is_some() && other.weekly_fraction.is_none() {
            return Some(core::cmp::Ordering::Less);
        } else if self.weekly_fraction.is_none() && other.weekly_fraction.is_some() {
            return Some(core::cmp::Ordering::Greater);
        }

        match self.weekly_fraction.partial_cmp(&other.weekly_fraction) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }

        self.time_periods.partial_cmp(&other.time_periods)
        /*
        match self.days.partial_cmp(&other.days) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        match self.sites.partial_cmp(&other.sites) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        match self.subspecialties.partial_cmp(&other.subspecialties) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        match self.contexts.partial_cmp(&other.contexts) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        self.modalities.partial_cmp(&other.modalities)
        */
    }
}

impl Ord for RotationResponsibility {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).expect("Bad ordering")
    }
}
impl Eq for RotationResponsibility {}

#[derive(Debug, PartialEq, Eq, PartialOrd, Clone)]
pub struct TimePeriods {
    value: Option<Vec<Timespan>>,
}

impl WrappedSortable<Timespan> for TimePeriods {
    fn get(&self) -> &Option<Vec<Timespan>> {
        &self.value
    }
    fn fromval(mut val: Option<Vec<Timespan>>) -> TimePeriods {
        match val {
            Some(mut x) => {
                x.sort();
                val = Some(x);
            }
            None => {
                ();
            }
        }

        TimePeriods { value: val }
    }
}

impl TimePeriods {
    pub fn from_strings(strings: Vec<&str>) -> Result<TimePeriods, RotationManifestParseError> {
        let mut periods: Vec<Timespan> = Vec::new();
        for str in strings {
            periods.push(Timespan::from_string(str)?);
        }
        Ok(TimePeriods {
            value: Some(periods),
        })
    }

    pub fn get(&self) -> &Option<Vec<Timespan>> {
        &self.value
    }
}

impl<'de> Deserialize<'de> for TimePeriods {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let mut val = Option::<Vec<Timespan>>::deserialize(deserializer)?;
        match val {
            Some(mut x) => {
                x.sort();
                val = Some(x);
            }
            None => {
                ();
            }
        }

        Ok(TimePeriods { value: val })
    }
}
impl Serialize for TimePeriods {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match &self.value {
            Some(x) => {
                let mut newvec: Vec<Timespan> = x.clone();
                newvec.sort();
                newvec.serialize(serializer)
            }
            None => serializer.serialize_none(),
        }
    }
}
