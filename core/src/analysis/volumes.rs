use std::collections::HashMap;

use chrono::NaiveDate;
use serde::Serialize;

#[derive(Serialize)]
pub struct CategorizedVolumes {
    date_map: HashMap<NaiveDate, HashMap<String, VolumesMark>>,
}

impl CategorizedVolumes {
    pub fn new() -> Self {
        Self {
            date_map: HashMap::new(),
        }
    }
    pub fn add(&mut self, date: NaiveDate, category: &str, new_mark: VolumesMark) -> () {
        match self.date_map.entry(date) {
            std::collections::hash_map::Entry::Occupied(mut entry) => {
                match entry.get_mut().entry(category.to_owned()) {
                    std::collections::hash_map::Entry::Occupied(mut entry) => {
                        entry.insert(new_mark + *entry.get());
                    }
                    std::collections::hash_map::Entry::Vacant(empty) => {
                        empty.insert(new_mark);
                    }
                }
            }
            std::collections::hash_map::Entry::Vacant(empty) => {
                let mut new_member: HashMap<String, VolumesMark> = HashMap::new();
                new_member.insert(category.to_owned(), new_mark);
                let _entry = empty.insert(new_member);
            }
        };
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Serialize)]
pub struct VolumesMark {
    pub rvu: f64,
    pub bvu: f64,
}

impl std::ops::Add for VolumesMark {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        VolumesMark {
            rvu: self.rvu + other.rvu,
            bvu: self.bvu + other.bvu,
        }
    }
}
