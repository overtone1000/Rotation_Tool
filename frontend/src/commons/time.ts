import type { MinutesSinceMidnight } from "../components/coverage/CoverageTree";

export const days_of_the_week = [
    "Sunday",
    "Monday",
    "Tuesday",
    "Wednesday",
    "Thursday",
    "Friday",
    "Saturday"
];

export const short_days_of_the_week = [
    "Sun",
    "Mon",
    "Tue",
    "Wed",
    "Thu",
    "Fri",
    "Sat"
];

export const pd = (di:number)=>{
    switch(di)
    {
        case 0: return 6;
        default: return di-1;
    }
}

export const pbd = (di:number)=>{
    switch(di)
    {
        case 0:
        case 1:
             return 5;
        default: return di-1;
    }
}

export const pbdp1 = (di:number)=>{
    return nd(pbd(di));
}

export const nd = (di:number)=>{
    switch(di)
    {
        case 6: return 0;
        default: return di+1;
    }
}

export const day_indices = [0,1,2,3,4,5,6];

export const dowfunc=(di:number)=>{
    console.debug("dowfunc",di);
    return days_of_the_week[di];
}

export const shortdowfunc=(di:number)=>{
    return short_days_of_the_week[di];
}

export const minutes_since_midnight_to_time_string=(msm:MinutesSinceMidnight)=>{
    const hrs = msm.minutes/60;
    const minutes = msm.minutes%60;
    return hrs.toString().padStart(2,"0") + ":" + minutes.toString().padStart(2,"0");
}