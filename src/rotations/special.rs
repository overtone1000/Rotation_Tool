pub mod TimeModifiers
{
    pub const previous_business_day:String="PBD".to_string();
    pub const next_business_day:String="NBD".to_string();
    pub const current_business_day:String="PBD".to_string();
}

pub mod Days
{
    pub const monday:String=chrono::Weekday::Mon.to_string();
    pub const tuesday:String=chrono::Weekday::Tue.to_string();
    pub const wednesday:String=chrono::Weekday::Wed.to_string();
    pub const thursday:String=chrono::Weekday::Thu.to_string();
    pub const friday:String=chrono::Weekday::Fri.to_string();
    pub const saturday:String=chrono::Weekday::Sat.to_string();
    pub const sunday:String=chrono::Weekday::Sun.to_string();
}