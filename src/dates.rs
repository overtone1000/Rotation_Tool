use chrono::{NaiveDate, Datelike};

pub fn checkWeekDay(date:NaiveDate)->bool
{
  match date.weekday()
  {
      chrono::Weekday::Sun=>false,
      chrono::Weekday::Sat=>false,
      _=>true
  }
}

fn calcNumberOfDayOfWeekInMonth(date:NaiveDate)->u32
{
  let day_of_week=date.weekday();
  let month=date.month(); //january is 1
  let year=date.year();

  let mut retval:u32=0;
  for n in 1..40
  {
    let this_date=NaiveDate::from_ymd_opt(year,month,n);
    match this_date
    {
        None=>{
            return retval;
        },
        Some(d)=>{
            if(d.month()!=month)
            {
                return retval;
            }
            if(d.weekday()==day_of_week)
            {
                retval+=1;
            }
        }
    }
  }
  return retval;
}


pub fn calcDayOfWeekOfMonth(date:NaiveDate)->u32
{
  let day_of_month=date.day();
  return (day_of_month as f32/7.0).ceil() as u32;
}

pub(crate) const business_days_per_year:f64=52.0*5.0-7.0;

fn holidayTest()->(){
    println!("Memorial day");
    checkHoliday(NaiveDate::from_ymd_opt(1999, 5,31).unwrap_or_default());
    println!("Independence day");
    checkHoliday(NaiveDate::from_ymd_opt(1999, 7,4).unwrap_or_default());
    println!("Not a holiday");
    checkHoliday(NaiveDate::from_ymd_opt(1999, 7,5).unwrap_or_default());
    println!("Christmas");
    checkHoliday(NaiveDate::from_ymd_opt(1999, 12,25).unwrap_or_default());
    println!("Labor day");
    checkHoliday(NaiveDate::from_ymd_opt(1999, 9,6).unwrap_or_default());
    println!("Thanksgiving");
    checkHoliday(NaiveDate::from_ymd_opt(1999, 11, 25).unwrap_or_default());
    println!("New Years");
    checkHoliday(NaiveDate::from_ymd_opt(1999, 1,1).unwrap_or_default());

}

pub fn checkHoliday(date:NaiveDate)->bool
{
    let day_of_week=date.weekday();
    let month=date.month(); //january is 1   
    let day_of_month=date.day(); //1st is 1

    //println!("Checking {} ({}) ({})",date.to_string(),calcDayOfWeekOfMonth(date),calcNumberOfDayOfWeekInMonth(date));

    //Memorial Day, last Monday of May
    if(month==chrono::Month::May.number_from_month() && day_of_week==chrono::Weekday::Mon && calcDayOfWeekOfMonth(date)==calcNumberOfDayOfWeekInMonth(date)){
      return true;
    }
    //Independence Day, July 4
    if(month==chrono::Month::July.number_from_month() && day_of_month==4){
      return true;
    }
    //Labor Day, first Monday of September
    if(month==chrono::Month::September.number_from_month() && day_of_week==chrono::Weekday::Mon && calcDayOfWeekOfMonth(date)==1){
      return true;
    }
    //Thanksgiving 4th Thursday in November
    if(month==chrono::Month::November.number_from_month() && day_of_week==chrono::Weekday::Thu && calcDayOfWeekOfMonth(date)==4){
      return true;
    }
    //Christmas December 25
    if(month==chrono::Month::December.number_from_month() && day_of_month==25){
      return true;
    }
    //New Years 1/1
    if(month==chrono::Month::January.number_from_month() && day_of_month==1){
      return true;
    }
    return false;
}