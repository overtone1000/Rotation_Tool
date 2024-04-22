import type { ValueData } from "./rotation_plot_data"

export interface Facility_Analysis_Data {
    [date_string:string]:{
        [facility_string:string]:ValueData
    }
};

export interface FacilityAnalysisMark {
    date:Date,
    facility:string,
    value:number
}