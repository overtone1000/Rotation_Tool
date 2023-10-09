pub(crate) const time_start_hour:i32=6;
pub(crate) const time_start_minute:i32=0;
pub(crate) const time_step_minutes:i32=30;
pub(crate) fn time_row_count()->i32{
    return (((24.0*60.0)/(time_step_minutes as f32))).floor() as i32;
}
pub(crate) fn getTimeRowIndex(hour:i32, minute:i32)->i32{
    let mut minute_of_day = hour*60+minute;
    let start_minute_of_day=time_start_hour*60+time_start_minute;
    if(minute_of_day<start_minute_of_day){minute_of_day+=24*60;}
    return (((minute_of_day-start_minute_of_day) as f32)/(time_step_minutes as f32)).floor() as i32;
}