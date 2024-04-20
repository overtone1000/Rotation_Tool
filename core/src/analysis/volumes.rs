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
    pub fn add(&mut self, date: NaiveDate, category: &str, new_mark: VolumesMark) {
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
    fn count_rotations(&self)->HashMap<String,u64>
    {
        let mut retval:HashMap<String,u64>=HashMap::new();
        for (_date,map) in &self.date_map
        {
            for (rotation,_) in map
            {
                match retval.entry(rotation.to_string())
                {
                    std::collections::hash_map::Entry::Occupied(mut occ) => {*occ.get_mut()+=1;},
                    std::collections::hash_map::Entry::Vacant(vac) => {vac.insert(1);},
                };
            }
        }
        retval
    }
}

impl core::fmt::Debug for CategorizedVolumes
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (rotation,count) in &self.count_rotations()
        {
            f.write_str(format!("{}:{}
            ",rotation,count).as_str())?;
        }
        Ok(())
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
