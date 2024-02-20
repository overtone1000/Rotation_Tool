<script lang="ts">
	import { key } from "../../commons/key";
	import type { AnalysisData, AnalysisMark } from "../../commons/plot_data";
	import { short_days_of_the_week_Mon_first } from "../../commons/time";
	import ObservablePlot from "./observable_plot.svelte";
	import * as Plot from "@observablehq/plot";

	type BuiltPlot = (SVGSVGElement | HTMLElement) & Plot.Plot;

	let data:AnalysisData|undefined=undefined;
	const get_plot_data=(file:string)=>{
		fetch(file).then(
			(value:Response)=>{
				if(value.ok)
				{
					value.json().then(
						(res:string)=>{
							data=JSON.parse(res) as AnalysisData;
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

    let valuetype:"bvu"|"rvu"="bvu";
	const rotation_filter = (d:AnalysisMark) => {
		return d.rotation==="BR/US";
	}

	const dow_filter = (d:string) => {
		return d==="Mon";
	}

	const build_plot=(data:AnalysisData, width:number, y:string, title:string)=>{

		//console.debug("Building plot with",data,data.series);
		let marks:any=[
			Plot.frame(),
			Plot.barY(
				data.marks.filter(rotation_filter),
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
				domain: short_days_of_the_week_Mon_first.filter(dow_filter)
			},
			x: {
				//domain: ["BR/US"]
			},
			marks: marks
		})
		return retval;
	}

	let plot_weekday_bvu:BuiltPlot|undefined=undefined;
	let plot_weekday_rvu:BuiltPlot|undefined=undefined;

	get_plot_data("data/rotation_by_weekday"+key+".json");

    let container_width:any="5000px";

	$:{
		if(data!==undefined)
		{
			plot_weekday_bvu=build_plot(data,container_width,valuetype,"Rotation Volume by Weekday");
		}
	}
</script>

<div class="container">
    <div bind:clientWidth={container_width}>
        {#if plot_weekday_bvu!==undefined}
            <ObservablePlot plot={plot_weekday_bvu}/>
        {/if}
    </div>
    <div class="controls">
        <div>
            RVU or BVU
        </div>
        <div>
            Filter DOW
        </div>
        <div>
            Filter Rotations
        </div>
    </div>
</div>

<style>
    .container {
		display: flex;
		flex-direction:row;
        width:100%;
	}
    .controls {
        display: flex;
        flex-direction: column;
        justify-content: space-evenly;
    }
</style>