import type * as Plot from "@observablehq/plot";

export type BuiltPlot = (SVGSVGElement | HTMLElement) & Plot.Plot;

export type ValueType = "bvu"|"rvu";

export const get_data=(file:string, callback:(raw_data_result:any)=>void)=>{
    fetch(file).then(
        (value:Response)=>{
            if(value.ok)
            {
                value.json().then(
                    (res:any)=>{
                        callback(res);
                    },
                    (err)=>{
                        console.error("Rejected promise.",err);
                    }
                );
            }
        },
        (err)=>{
            console.error("Rejected fetch",err);
        }
    );
};
