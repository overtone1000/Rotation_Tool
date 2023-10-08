use chrono::{NaiveDate, Datelike};

pub fn checkWeekDay(date:NaiveDate)->bool
{
    match date.weekday()
    {
        Sun=>true,
        Sat=>true,
        (_)=>false
    }
}

fn calcNumberOfDayOfWeekInMonth(date:NaiveDate)->u32
{
  let day_of_week=date.weekday();
  let month=date.month0(); //january is 0
  let day_of_month=date.day0()+1; //1st is 1
  let year=date.year();

  let mut retval:u32=0;
  for n in 0..40
  {
    let this_date=NaiveDate::from_ymd_opt(year,month,n);
    match this_date
    {
        None=>{
            return retval;
        },
        Some(d)=>{
            if(d.month0()!=month)
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

pub fn checkHoliday(date:NaiveDate)->bool
{

}