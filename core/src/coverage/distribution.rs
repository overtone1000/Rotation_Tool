use std::collections::HashMap;

use chrono::Duration;
use chrono::NaiveTime;
use statrs::distribution::ContinuousCDF;
use statrs::distribution::Normal;

pub(crate) const TIME_START_HOUR: u32 = 6;
pub(crate) const TIME_START_MINUTE: u32 = 0;
pub(crate) const TIME_STEP_MINUTES: u32 = 30;

pub(crate) fn time_row_count() -> usize {
    ((24.0 * 60.0) / (TIME_STEP_MINUTES as f32)).floor() as usize
}

pub(crate) fn get_time_row_index(hour: u32, minute: u32) -> usize {
    let mut minute_of_day = hour * 60 + minute;
    let start_minute_of_day = TIME_START_HOUR * 60 + TIME_START_MINUTE;
    if minute_of_day < start_minute_of_day {
        minute_of_day += 24 * 60;
    }
    (((minute_of_day - start_minute_of_day) as f32) / (TIME_STEP_MINUTES as f32)).floor() as usize
}

pub(crate) fn get_time_row_normal_dist_weights() -> HashMap<usize, f64> {
    let mut retval: HashMap<usize, f64> = HashMap::new();

    //Bounds, inclusive
    let start_i = get_time_row_index(8, 0);
    let stop_i = get_time_row_index(16, 30);

    let mean = ((stop_i + start_i) as f64) / 2.0;
    let std_dev = ((stop_i + start_i) as f64) / 4.0;

    let normal: Normal = Normal::new(mean, std_dev).unwrap(); //12 o'clock mean, 2 hr standard deviation

    let mut last: f64 = normal.cdf(start_i as f64);
    retval.insert(start_i, last);

    for n in (start_i + 1)..stop_i {
        let current = normal.cdf(n as f64);
        retval.insert(n, current - last);
        last = current;
    }
    retval.insert(stop_i, 1.0 - last);

    retval
}

pub(crate) fn get_normal_dist_weights() -> HashMap<NaiveTime, f64> {
    let mut retval: HashMap<NaiveTime, f64> = HashMap::new();

    //Bounds, inclusive
    let start_i = NaiveTime::from_hms_opt(8, 0, 0).expect("Should be a valid time.");
    let stop_i = NaiveTime::from_hms_opt(16, 30, 0).expect("Should be a valid time.");
    let midnight = NaiveTime::from_hms_opt(0, 0, 0).expect("Should be a valid time.");

    let start_minutes = (start_i - midnight).num_minutes();
    let stop_minutes = (stop_i - midnight).num_minutes();

    let mean = ((start_minutes + stop_minutes) as f64) / 2.0;
    let std_dev = ((start_minutes + stop_minutes) as f64) / 4.0;

    let normal: Normal = Normal::new(mean, std_dev).unwrap(); //12 o'clock mean, 2 hr standard deviation

    let mut last: f64 = normal.cdf(start_minutes as f64);
    retval.insert(start_i, last);

    for n in (start_minutes + 1)..stop_minutes {
        let current = normal.cdf(n as f64);
        let time = midnight + Duration::minutes(n);
        retval.insert(time, current - last);
        last = current;
    }
    retval.insert(stop_i, 1.0 - last);

    retval
}
