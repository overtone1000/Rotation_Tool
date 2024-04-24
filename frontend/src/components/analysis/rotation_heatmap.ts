import * as Plot from "@observablehq/plot";
import type { AnalysisMark, Rotation_Analysis_Data } from "../../commons/rotation_plot_data";
import type { ValueType } from "./commons";

export const get_rotation_marks=(data:Rotation_Analysis_Data)=>{
    let all_marks:AnalysisMark[]=[];
    for(let date_string in data.date_map)
    {
        let date_data=data.date_map[date_string];
        let date=new Date(date_string);
        for(let rotation in date_data)
        {
            let rotation_data=date_data[rotation];
            all_marks.push(
                {
                    date:date,
                    rotation:rotation,
                    value:rotation_data
                }
            );
        }
    }
    
    return all_marks;		
}

export const build_heatmap=(analysis_marks:AnalysisMark[], width:number, options:{title:string, valuetype:ValueType})=>{
    let max=0;
    let min=0;
    for(let mark of analysis_marks)
    {
        if(mark.value[options.valuetype]>max){max=mark.value[options.valuetype];}
        if(mark.value[options.valuetype]<min){min=mark.value[options.valuetype];}
    }

    let marks:any=[
        Plot.rect(
            analysis_marks,
            Plot.binY(
                {
                    fill:"proportion-facet",
                    members:"identity"
                },
                {
                    y: (d)=>d.value[options.valuetype],
                    fx: "rotation",
                    inset: 0,  
                } as Plot.BinYInputs<Plot.RectOptions>
            )
        ),
        /*
        Plot.tip(
            displayed_marks,
            Plot.binY(
                {
                    fill:"count",
                },
                Plot.pointer(workaround_tips)
            )
        ),
        */
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