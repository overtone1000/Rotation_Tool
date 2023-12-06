use chrono::{Datelike, NaiveDate};



pub fn check_week_day(date: NaiveDate) -> bool {
    match date.weekday() {
        chrono::Weekday::Sun => false,
        chrono::Weekday::Sat => false,
        _ => true,
    }
}

fn calc_number_of_day_of_week_in_month(date: NaiveDate) -> u32 {
    let day_of_week = date.weekday();
    let month = date.month(); //january is 1
    let year = date.year();

    let mut retval: u32 = 0;
    for n in 1..40 {
        let this_date = NaiveDate::from_ymd_opt(year, month, n);
        match this_date {
            None => {
                return retval;
            }
            Some(d) => {
                if d.month() != month {
                    return retval;
                }
                if d.weekday() == day_of_week {
                    retval += 1;
                }
            }
        }
    }
    retval
}

pub fn calc_day_of_week_of_month(date: NaiveDate) -> u32 {
    let day_of_month = date.day();
    (day_of_month as f32 / 7.0).ceil() as u32
}

pub(crate) const BUSINESS_DAYS_PER_YEAR: f64 = 52.0 * 5.0 - 7.0;

fn holiday_test() {
    println!("Memorial day");
    check_holiday(NaiveDate::from_ymd_opt(1999, 5, 31).unwrap_or_default());
    println!("Independence day");
    check_holiday(NaiveDate::from_ymd_opt(1999, 7, 4).unwrap_or_default());
    println!("Not a holiday");
    check_holiday(NaiveDate::from_ymd_opt(1999, 7, 5).unwrap_or_default());
    println!("Christmas");
    check_holiday(NaiveDate::from_ymd_opt(1999, 12, 25).unwrap_or_default());
    println!("Labor day");
    check_holiday(NaiveDate::from_ymd_opt(1999, 9, 6).unwrap_or_default());
    println!("Thanksgiving");
    check_holiday(NaiveDate::from_ymd_opt(1999, 11, 25).unwrap_or_default());
    println!("New Years");
    check_holiday(NaiveDate::from_ymd_opt(1999, 1, 1).unwrap_or_default());
}

pub fn check_holiday(date: NaiveDate) -> bool {
    let day_of_week = date.weekday();
    let month = date.month(); //january is 1
    let day_of_month = date.day(); //1st is 1

    //println!("Checking {} ({}) ({})",date.to_string(),calcDayOfWeekOfMonth(date),calcNumberOfDayOfWeekInMonth(date));

    //Memorial Day, last Monday of May
    if month == chrono::Month::May.number_from_month()
        && day_of_week == chrono::Weekday::Mon
        && calc_day_of_week_of_month(date) == calc_number_of_day_of_week_in_month(date)
    {
        return true;
    }
    //Independence Day, July 4
    if month == chrono::Month::July.number_from_month() && day_of_month == 4 {
        return true;
    }
    //Labor Day, first Monday of September
    if month == chrono::Month::September.number_from_month()
        && day_of_week == chrono::Weekday::Mon
        && calc_day_of_week_of_month(date) == 1
    {
        return true;
    }
    //Thanksgiving 4th Thursday in November
    if month == chrono::Month::November.number_from_month()
        && day_of_week == chrono::Weekday::Thu
        && calc_day_of_week_of_month(date) == 4
    {
        return true;
    }
    //Christmas December 25
    if month == chrono::Month::December.number_from_month() && day_of_month == 25 {
        return true;
    }
    //New Years 1/1
    if month == chrono::Month::January.number_from_month() && day_of_month == 1 {
        return true;
    }
    false
}
