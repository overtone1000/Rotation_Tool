use std::arch::global_asm;

use serde::{Serialize, Deserialize};

use crate::globals;

use super::stringtypes::StringTypes;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct RotationResponsibility
{
    pub sites:StringTypes,
    pub subspecialties:StringTypes,
    pub contexts:StringTypes,
    pub modalities:StringTypes,
    pub days:StringTypes,
    pub time_periods:StringTypes,
    pub(crate) comments:Option<Vec<String>>
}

impl RotationResponsibility
{
    pub fn validate(&self)->Result<(),Vec<String>>
    {
        let mon = chrono::Weekday::Mon.to_string();
        let tue = chrono::Weekday::Tue.to_string();
        let wed = chrono::Weekday::Wed.to_string();
        let thu = chrono::Weekday::Thu.to_string();
        let fri = chrono::Weekday::Fri.to_string();
        let sat = chrono::Weekday::Sat.to_string();
        let sun  = chrono::Weekday::Sun.to_string();

        let days = &[
            mon.as_str(),
            tue.as_str(),
            wed.as_str(),
            thu.as_str(),
            fri.as_str(),
            sat.as_str(),
            sun.as_str()
        ];        

        let mut errors:Vec<String> = Vec::new();

        let mut check = |t:&StringTypes,poss:&[&str],desc:&str| ->() {
            match t.validate(poss)
            {
                Err(e) => {
                    for i in e
                    {
                        errors.push(format!("Invalid {} {}. Valid values are {:?}",desc,i,poss));
                    }
                },
                _=>()
            };
        };

        check(&self.sites,globals::SITES,"site");
        check(&self.subspecialties,globals::SUBSPECIALTIES,"subspecialty");
        check(&self.contexts,globals::CONTEXTS,"context");
        check(&self.modalities,globals::MODALITIES,"modality");
        check(&self.days,days,"weekday");

        if errors.len()>0
        {
            Err(errors)
        }
        else {
            Ok(())
        }
    }
}