use std::{
    collections::{hash_map::Entry, HashMap},
    ops::AddAssign,
};

use chrono::NaiveDateTime;
use serde::Serialize;

#[derive(Default, Debug, Clone)]
pub struct SerializeableNaiveDateTime {
    pub datetime: NaiveDateTime,
}
impl Serialize for SerializeableNaiveDateTime {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_i64(self.datetime.timestamp())
    }
}

#[derive(Default, Debug, Serialize, Clone)]
pub struct WorkUnit {
    datetime: SerializeableNaiveDateTime,
    rvu: f64,
    bvu: f64,
    exam_desc: String,
    //denominator: f64, //Used only for fractional type? No, was using in work adding! Get rid of this.
}

impl WorkUnit {
    pub fn get_datetime(&self) -> NaiveDateTime {
        self.datetime.datetime
    }
    pub fn get_absolute_rvu(&self)->f64{
        self.rvu
    }
    /*
    pub fn get_scaled_rvu(&self) -> f64 {
        self.rvu / self.denominator
    }
    pub fn get_scaled_bvu(&self) -> f64 {
        self.bvu / self.denominator
    }
     */
    pub fn get_absolute_bvu(&self)->f64{
        self.bvu
    }
    pub fn get_exam_desc(&self) -> &str {
        self.exam_desc.as_str()
    }
    pub fn create(
        datetime: NaiveDateTime,
        rvu: f64,
        bvu: f64,
        //denominator: f64,
        exam_desc: String,
    ) -> WorkUnit {
        WorkUnit {
            datetime: SerializeableNaiveDateTime { datetime },
            rvu,
            bvu,
            exam_desc,
            //denominator,
        }
    }
}

#[derive(Debug, Serialize, Default, Clone)]
pub struct AnalysisDatum {
    total_rvu: f64,
    total_bvu: f64,
    study_counts_by_exam_code: HashMap<String, f64>,
}

impl AddAssign for AnalysisDatum {
    fn add_assign(&mut self, rhs: Self) {
        self.total_rvu += rhs.total_rvu;
        self.total_bvu += rhs.total_bvu;

        for (rhs_key, rhs_val) in rhs.study_counts_by_exam_code {
            self.add_studies(rhs_key, rhs_val);
        } 
    }
}


#[derive(Debug, Serialize, Default, Clone)]
pub struct ComparisonDatum {
    pub rvu:f64,
    pub bvu:f64
}

impl AnalysisDatum {
    pub fn get_rvu(&self) -> f64 {
        self.total_rvu
    }
    pub fn get_bvu(&self) -> f64 {
        self.total_bvu
    }
    pub fn get_studies(&self) -> &HashMap<String, f64> {
        &self.study_counts_by_exam_code
    }

    pub fn scale(&mut self, scale: f64) {
        self.total_rvu *= scale;
        self.total_bvu *= scale;

        for (_, val) in &mut self.study_counts_by_exam_code {
            *val *= scale;
        }
    }

    pub fn add_workunit(&mut self, rhs: &WorkUnit) {
        self.total_rvu += rhs.get_absolute_rvu();
        self.total_bvu += rhs.get_absolute_bvu();
        self.add_studies(rhs.exam_desc.to_string(), 1.0);// / rhs.denominator);
    }

    fn add_studies(&mut self, key: String, val: f64) {
        match self.study_counts_by_exam_code.entry(key) {
            Entry::Occupied(mut o) => {
                let curval = o.get_mut();
                *curval += val;
            }
            Entry::Vacant(v) => {
                v.insert(val);
            }
        }
    }
}
