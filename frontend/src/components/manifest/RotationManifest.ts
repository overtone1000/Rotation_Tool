import type { ValueData } from "../../commons/rotation_plot_data";

export interface RotationManifest
{
    title:string,
    rotation_manifest:[Rotation],
    baselines:[Baseline]
}

export interface WorkHoursPeriod
{hours:TimePeriodString,days:Weekday[]}

export interface Rotation
{
    rotation:string,
    location:string,
    hours?:WorkHoursPeriod[],
    breaktime?:[TimePeriodString,string]
    responsibilities:Responsibility[],
    comments:string[]
}

export interface Responsibility
{
    sites:string|string[],
    contexts:string|string[],
    days:Weekday|Weekday[]|"All",
    //modalities:string|string[],
    exams:string|string[],
    time_periods:null|[TimePeriodString],
    weekly_fraction:null|number,
    volume?:ValueData
}

export type TimePeriodString = string;
export type DayOffset = "CD" | "PD" | "PBD" | "PBD+1" | "ND";
export const all_dayoffsets:DayOffset[] = [
    "CD",
    "PD",
    "PBD",
    "PBD+1",
    "ND"
]
export function dayoffsetToDisplayString(dayoffset:DayOffset)
{
    switch(dayoffset)
    {
        case "CD":return "Current Day";
        case "PD":return "Prior Day";
        case "PBD":return "Prior Business Day";
        case "PBD+1":return "Day After Prior Business Day";
        case "ND":return "Next Day";
    }
}
export interface RelativeTime {
    time:string, //format hh:mm
    day:DayOffset,
}
export interface TimePeriod {
    start:RelativeTime,
    end:RelativeTime
}
export function parseRelativeTimeString(str:string)
{
    let contents=str.split(" ");
    let hourminute=contents[0].split(":");
    let retval:RelativeTime={
        time: hourminute[0].padStart(2,"0")+":"+hourminute[1].padStart(2,"0"),
        day: contents[1] as DayOffset
    }
    return retval;
}
export function relativeTimeToString(rt:RelativeTime)
{
    return rt.time + " " + rt.day;
}
export function parseTimePeriodString(str:TimePeriodString)
{
    let relative_times=str.split("-");
    let retval:TimePeriod={
        start:parseRelativeTimeString(relative_times[0]),
        end:parseRelativeTimeString(relative_times[1])
    };
    return retval;
}
export function timePeriodToString(period:TimePeriod)
{
    return relativeTimeToString(period.start) + "-" + relativeTimeToString(period.end);
}

export interface Baseline
{
    rotation:string,
    rvu:number,
    bvu:number
}

export type Weekday = "Mon" | "Tue" | "Wed" | "Thu" | "Fri" | "Sat" | "Sun";
export const all_weekdays:Weekday[]=[
    "Mon",
    "Tue",
    "Wed",
    "Thu",
    "Fri",
    "Sat",
    "Sun"
];