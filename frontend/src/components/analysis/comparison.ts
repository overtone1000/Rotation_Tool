import * as Plot from "@observablehq/plot";
import type { ValueType } from "./commons";

interface ComparisonDatum{
    rvu:number,
    bvu:number
}

type Facet="active"|"proposed"

type Differential_Data = {
    [facet in Facet]: { [rotation: string]: ComparisonDatum; };
};

interface ComparisonMark{
    facet:Facet
    rotation:string,
    value:ComparisonDatum
}

export const get_comparison_marks=(data:Differential_Data)=>{
    let retval:ComparisonMark[]=[];
    for(const facet in data)
    {
        let subdata=data[facet as Facet];
        for(const rotation in subdata)
        {
            let datum=subdata[rotation];
            retval.push(
                {
                    facet:facet as Facet,
                    rotation:rotation,
                    value:datum
                }
            );   
        }
    }
    return retval;
}

export const build_comparison=(comparison_marks:ComparisonMark[], width:number, options:{title:string, valuetype:ValueType, rotations:string[]})=>{
    
    let filtered_marks:ComparisonMark[]=comparison_marks.filter(
        (mark)=>(options.rotations.includes(mark.rotation))
    );

    let marks:any=[
        Plot.barY(
            filtered_marks,
            {
                y: (d)=>d.value[options.valuetype],
                fx: "rotation",
                x: "facet",
                fill: "facet"
            }
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
        padding: 0.1,
        //fx:{
        //	domain: displayed_marks.filter((d)=>d.rotation)
        //},
        marginBottom: 120,
        marginTop: 120,
        marginLeft: 80,
        x:{
            grid:true,
            label:"Rotation",
            tickRotate:-45
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