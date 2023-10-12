use std::collections::HashMap;

use statrs::distribution::Normal;
use statrs::distribution::ContinuousCDF;

pub(crate) const time_start_hour:u32=6;
pub(crate) const time_start_minute:u32=0;
pub(crate) const time_step_minutes:u32=30;

pub(crate) fn time_row_count()->usize{
    return (((24.0*60.0)/(time_step_minutes as f32))).floor() as usize;
}

pub(crate) fn getTimeRowIndex(hour:u32, minute:u32)->usize{
    let mut minute_of_day = hour*60+minute;
    let start_minute_of_day=time_start_hour*60+time_start_minute;
    if(minute_of_day<start_minute_of_day){minute_of_day+=24*60;}
    return (((minute_of_day-start_minute_of_day) as f32)/(time_step_minutes as f32)).floor() as usize;
}

pub(crate) fn getTimeRowNormalDistWeights()->HashMap<usize,f64>{
    let mut retval:HashMap<usize,f64>=HashMap::new();

    //Bounds, inclusive
    let start_i=getTimeRowIndex(8,0);
    let stop_i=getTimeRowIndex(16,30);

    let mean = ((stop_i+start_i) as f64)/2.0;
    let std_dev = ((stop_i+start_i) as f64)/4.0;

    let normal: Normal = Normal::new(mean,std_dev).unwrap(); //12 o'clock mean, 2 hr standard deviation   
    
    let mut last:f64=normal.cdf(start_i as f64);
    retval.insert(start_i,last);
    
    for n in (start_i+1)..stop_i
    {
        let current=normal.cdf(n as f64);
        retval.insert(n,current-last);
        last=current;
    }
    retval.insert(stop_i,1.0-last);

    return retval;
}