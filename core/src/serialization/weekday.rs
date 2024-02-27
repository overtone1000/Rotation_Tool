use serde::Serialize;

use crate::coverage::units::temporal_coverage::weekday_for_javascript;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct SerializeableWeekday {
    pub day: chrono::Weekday,
}

impl SerializeableWeekday {
    pub fn new(weekday: chrono::Weekday) -> SerializeableWeekday {
        SerializeableWeekday { day: weekday }
    }
}

impl Serialize for SerializeableWeekday {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_u32(weekday_for_javascript(&self.day))
    }
}

impl PartialOrd for SerializeableWeekday {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.day
            .num_days_from_sunday()
            .partial_cmp(&other.day.num_days_from_sunday())
    }
}

impl Ord for SerializeableWeekday {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.partial_cmp(other) {
            Some(x) => x,
            None => std::cmp::Ordering::Equal,
        }
    }
}

impl Default for SerializeableWeekday {
    fn default() -> Self {
        Self { day: chrono::Weekday::Sun }
    }
}