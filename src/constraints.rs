use chrono::{NaiveDateTime, NaiveDate, Datelike, Timelike};

use crate::{dates, rvu_map::MapCoords, globals::{TPC, Outpatient, NEURO_OTHER, NEURO_BRAIN, MSK}};

pub struct ConstraintSet<'a, T>{
    constraints:Vec<&'a dyn Fn(&T)->bool>
}
impl<'a, T> ConstraintSet<'a, T>{
    pub fn new()->ConstraintSet<'a, T>{
        ConstraintSet{
            constraints:Vec::new()
        }
    }

    pub fn include(&self, t:&T)->bool{
        for constraint in &self.constraints
        {
            if !constraint(t) {return false;}
        }
        true
    }

    pub fn add(&mut self, fun:&'a impl Fn(&T)->bool)
    {
        self.constraints.push(fun);
    }
}

pub(crate) fn is_not_holiday(datetime:&NaiveDateTime)->bool{
    !dates::checkHoliday(NaiveDate::from(*datetime))    
}

pub(crate) fn is_weekday(datetime:&NaiveDateTime)->bool{
    dates::checkWeekDay(NaiveDate::from(*datetime))    
}


pub(crate) fn is_this_day<'a>(day:chrono::Weekday)->impl Fn(&NaiveDateTime)->bool{
     move |datetime:&NaiveDateTime| {
        datetime.weekday()==day
    }
}

pub(crate) fn exclude_site(site:String)->impl Fn(&MapCoords)->bool{
    move |coords:&MapCoords| {
        coords.getSite()!=&site
    }
}

pub(crate) fn only_this_context(context:String)->impl Fn(&MapCoords)->bool{
    move |coords:&MapCoords| {
        coords.getContext()==&context
    }
}

pub(crate) fn only_these_subspecialties(subspecialties:Vec<String>)->impl Fn(&MapCoords)->bool{
    move |coords:&MapCoords| {
        for subspecialty in &subspecialties
        {
            if coords.getSubspecialty()==subspecialty {return true;}
        }
        false
    }
}

pub(crate) fn is_before_this_hour(hour:u32)->impl Fn(&NaiveDateTime)->bool{
    move |datetime:&NaiveDateTime| {
        datetime.hour()<hour
    }
}

pub(crate) fn is_after_this_hour(hour:u32)->impl Fn(&NaiveDateTime)->bool{
    move |datetime:&NaiveDateTime| {
        !(is_before_this_hour(hour)(datetime))
    }
}


pub fn is_business_day<'a>()->ConstraintSet<'a,NaiveDateTime>{
    let mut is_business_day:ConstraintSet<'a,NaiveDateTime>=ConstraintSet::new();
    let is_not_holiday_ref=&is_not_holiday;
    let is_weekday_ref=&is_weekday;
    is_business_day.add(is_not_holiday_ref);
    is_business_day.add(is_weekday_ref);
    is_business_day
  }