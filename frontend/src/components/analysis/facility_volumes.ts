import * as Plot from "@observablehq/plot";
import type { FacilityAnalysisMark, Facility_Analysis_Data } from "../../commons/facility_plot_data";
import type { ValueType } from "./commons";

export const get_facility_marks=(data:Facility_Analysis_Data)=>{
    let all_marks:FacilityAnalysisMark[]=[];
    for(let date_string in data)
    {
        let date_data=data[date_string];
        let date=new Date(date_string);
        for(let facility in date_data)
        {
            let facility_data=date_data[facility];
            all_marks.push(
                {
                    date:date,
                    facility:facility,
                    value:facility_data
                }
            );
        }
    }
    
    return all_marks;		
}

export const build_site_plot=(facility_marks:FacilityAnalysisMark[], width:number, options:{title:string, valuetype:ValueType})=>{

    let marks:any=[
        Plot.barY(
            facility_marks,
            Plot.groupX(
                {
                    y:"sum"
                },
                {
                    y:(d)=>d.value[options.valuetype],
                    x:"date",
                    fill:"facility",
                    order:"sum",
                    reverse:false
                }
            )
        ),
    ];

    const retval = Plot.plot({
        title: options.title,
        color: {legend:true},
        width: width,
        //aspectRatio: 1,
        height: 600,
        y:{
            grid:false,
            label:options.valuetype
        },
        padding: 0,
        //fx:{
        //	domain: displayed_marks.filter((d)=>d.rotation)
        //},
        marginBottom: 120,
        marginLeft: 80,
        x:{
            grid:true,
            label:"Rotation",
        },
        fx:{
            tickRotate:-45,
        },
        marks: marks,
        style:{
            fontSize:"14px"
        }
    })
    return retval;
}