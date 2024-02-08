export interface RotationManifest
{
    title:string,
    rotation_manifest:[Rotation],
    baselines:[Baseline]
}

export interface WorkHoursPeriod
{hours:TimePeriod,days:Weekday[]}

export interface Rotation
{
    rotation:string,
    location:string,
    hours?:WorkHoursPeriod[],
    breaktime?:[TimePeriod,string]
    responsibilities:[Responsibility],
    comments:string[]
}

export interface Responsibility
{
    sites:string|string[],
    contexts:string|string[],
    days:Weekday|Weekday[],
    //modalities:string|string[],
    exams:string|string[],
    time_periods:null|[TimePeriod],
    weekly_fraction:null|number
}

export type TimePeriod = string;

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