export interface RotationManifest
{
    title:string,
    rotation_manifest:[Rotation],
    baslines:[Baseline]
}

export interface Rotation
{
    rotation:string,
    location:string,
    responsibilities:[Responsibility],
    comments:string[]
}

export interface Responsibility
{
    sites:string|string[],
    contexts:string|string[],
    days:string|string[],
    modalities:string|string[],
    subspecialties:string|string[],
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