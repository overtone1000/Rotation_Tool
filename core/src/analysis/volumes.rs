use std::collections::{BTreeMap};

use chrono::{Datelike, NaiveDate};
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct CategorizedVolumes {
    date_map: BTreeMap<NaiveDate, BTreeMap<String, VolumesMark>>,
}

impl CategorizedVolumes {
    pub fn new() -> Self {
        Self {
            date_map: BTreeMap::new(),
        }
    }
    pub fn add(&mut self, date: NaiveDate, category: &str, new_mark: VolumesMark) {
        match self.date_map.entry(date) {
            std::collections::btree_map::Entry::Occupied(mut entry) => {
                match entry.get_mut().entry(category.to_owned()) {
                    std::collections::btree_map::Entry::Occupied(mut entry) => {
                        entry.insert(new_mark + *entry.get());
                    }
                    std::collections::btree_map::Entry::Vacant(empty) => {
                        empty.insert(new_mark);
                    }
                }
            }
            std::collections::btree_map::Entry::Vacant(empty) => {
                let mut new_member: BTreeMap<String, VolumesMark> = BTreeMap::new();
                new_member.insert(category.to_owned(), new_mark);
                let _entry = empty.insert(new_member);
            }
        };
    }
    fn count_rotations(&self)->BTreeMap<String,u64>
    {
        let mut retval:BTreeMap<String,u64>=BTreeMap::new();
        for (_date,map) in &self.date_map
        {
            for (rotation,_) in map
            {
                match retval.entry(rotation.to_string())
                {
                    std::collections::btree_map::Entry::Occupied(mut occ) => {*occ.get_mut()+=1;},
                    std::collections::btree_map::Entry::Vacant(vac) => {vac.insert(1);},
                };
            }
        }
        retval
    }
    pub fn retain<T>(&mut self, func:T)
    where T:FnMut(&NaiveDate, &mut BTreeMap<std::string::String, VolumesMark>)->bool
    {
        self.date_map.retain(func);
    }
}

impl core::fmt::Debug for CategorizedVolumes
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(format!("Categorized Volumes:
        ").as_str())?;
        
        for (rotation,count) in &self.count_rotations()
        {
            f.write_str(format!("{}:{}
            ",rotation,count).as_str())?;
        }
        f.write_str(format!("
        ").as_str())?;

        for (date,map) in &self.date_map
        {
            let mut rotation_string:String="".to_string();
            for (rotation,_) in map
            {
                rotation_string+=rotation;
                rotation_string+=" ";
            }

            f.write_str(format!(
                "{}, {}: {}
                ",
                date.weekday(),
                date.to_string().as_str(),
                rotation_string.as_str()
            ).as_str())?;
        }

        f.write_str(format!("Outliers:
        ").as_str())?;

        let mut sorted_rotation_and_date:Vec<(&NaiveDate,&str,&VolumesMark)>=Vec::new();
        for (date,map) in &self.date_map
        {
            for (rotation, mark) in map
            {
                if 
                    rotation=="BR/US" ||
                    rotation=="MSK" ||
                    rotation=="NM" ||
                    rotation=="SC-Main" ||
                    rotation=="WVH"
                {
                    sorted_rotation_and_date.push((date,rotation,mark));
                }
            }
        }
        sorted_rotation_and_date.sort_by(
        |first,second|
        {
            match first.2.rvu.total_cmp(&second.2.rvu)
            {
                std::cmp::Ordering::Equal => {
                    first.2.bvu.total_cmp(&second.2.bvu)
                },
                x => x,
            }
        });

        let mut outlier_count=100;
        if outlier_count>sorted_rotation_and_date.len() {outlier_count=sorted_rotation_and_date.len();}

        f.write_str(format!("Lowest outliers:
        ").as_str())?;
        for (date, rotation, mark) in &sorted_rotation_and_date[..outlier_count]
        {
            f.write_str(format!(
                "{}, {}: {} ({} rvu, {} bvu)
                ",
                date.weekday(),
                date.to_string().as_str(),
                rotation,
                mark.rvu,
                mark.bvu
            ).as_str())?;
        }



        Ok(())
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
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

impl std::ops::AddAssign for VolumesMark {
    fn add_assign(&mut self, rhs: Self) {
        self.rvu+=rhs.rvu;
        self.bvu+=rhs.bvu;
    }
}