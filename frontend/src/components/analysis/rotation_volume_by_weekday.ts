import * as Plot from "@observablehq/plot";
import type { AnalysisMark, Rotation_Analysis_Data } from "../../commons/rotation_plot_data";
import { short_days_of_the_week_Mon_first } from "../../commons/time";

export const build_weekday_plot=(displayed_marks:AnalysisMark[], width:number, y:string, title:string)=>{	

    let marks:any=[
        Plot.frame(),
        Plot.barY(
            displayed_marks,
            {	
                x:"rotation",
                y:y,
                fx:"weekday",
                fill:"rotation",
                stroke:"rotation",
                sort: {
                    x: "y"
                }
            }
        ),
        Plot.axisX({
            label: "Rotation",
            text: null,
            ticks: []
        }),
        Plot.axisFx({
            label: "Weekday"
        }),
    ];

    const retval = Plot.plot({
        title: title,
        color: {legend:true},
        width: width,
        //aspectRatio: 1,
        height: 600,
        fx: {
            domain: short_days_of_the_week_Mon_first
        },
        x: {
            //domain: ["BR/US"]
        },
        marks: marks
    })
    return retval;
}