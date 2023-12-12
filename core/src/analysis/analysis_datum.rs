use std::{ops::AddAssign, collections::{HashMap, hash_map::Entry}};

use chrono::NaiveDateTime;

#[derive(Default, Debug)]
pub struct WorkUnit {
    datetime: NaiveDateTime,
    rvu: f64,
    bvu: f64,
    exam_desc: String,
    denominator: f64
}

impl WorkUnit {
    pub fn get_datetime(&self)->NaiveDateTime{self.datetime}
    pub fn get_scaled_rvu(&self)->f64{self.rvu/self.denominator}
    pub fn get_scaled_bvu(&self)->f64{self.bvu/self.denominator}
    pub fn get_exam_desc(&self)->&str{self.exam_desc.as_str()}
    pub fn create(
        datetime:NaiveDateTime,
        rvu:f64,
        bvu:f64,
        denominator:f64,
        exam_desc:String
    )->WorkUnit
    {
        WorkUnit { 
            datetime: datetime, 
            rvu: rvu, 
            bvu: bvu, 
            exam_desc: exam_desc, 
            denominator: denominator }
    }
}

#[derive(Default, Debug)]
pub struct AnalysisDatum {
    total_rvu: f64,
    total_bvu: f64,
    studies: HashMap<String,f64>
}

impl AddAssign for AnalysisDatum {
    fn add_assign(&mut self, rhs: Self) {
        self.total_rvu += rhs.total_rvu;
        self.total_bvu += rhs.total_bvu;

        for (rhs_key, rhs_val) in rhs.studies
        {
            self.add_studies(rhs_key, rhs_val);
        }
    }
}

impl AnalysisDatum {
    pub fn get_rvu(&self)->f64{self.total_rvu}
    pub fn get_bvu(&self)->f64{self.total_bvu}
    pub fn get_studies(&self)->&HashMap<String,f64>{&self.studies}

    pub fn scale(&mut self, scale: f64) {
        self.total_rvu *= scale;
        self.total_bvu *= scale;

        for (_,val) in &mut self.studies
        {
            *val *= scale;
        }
    }

    pub fn add_workunit(&mut self, rhs: &WorkUnit) {
        self.total_rvu += rhs.get_scaled_rvu();
        self.total_bvu += rhs.get_scaled_bvu();
        self.add_studies(rhs.exam_desc.to_string(), 1.0/rhs.denominator);
    }

    fn add_studies(&mut self, key:String, val:f64)
    {
        match self.studies.entry(key)
        {
            Entry::Occupied(mut o) => {
                let curval = o.get_mut();
                *curval+=val;
            },
            Entry::Vacant(v) => {
                v.insert(val);
            },
        }
    }
}
