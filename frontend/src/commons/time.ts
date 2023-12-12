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

export const day_indices = [0,1,2,3,4,5,6];

export const dowfunc=(di:number)=>{
    return days_of_the_week[di];
}

export const shortdowfunc=(di:number)=>{
    return short_days_of_the_week[di];
}