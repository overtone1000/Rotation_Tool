<script lang="ts">
	import { key } from "../../commons/key";
	import type { AnalysisData } from "../../commons/plot_data";
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

	const build_plot=(data:AnalysisData, width:number, y:string, title:string)=>{

		//console.debug("Building plot with",data,data.series);
		let marks:any=[
			Plot.frame(),
			Plot.barY(
				data.marks,
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
			})
		];

		const retval = Plot.plot({
			title: title,
			color: {legend:true},
			width: width,
			//aspectRatio: 1,
			height: 600,
			fx: {
				domain: ["Mon","Tue","Wed","Thu","Fri","Sat","Sun"]
			},
			marks: marks
		})
		return retval;
	}

	let plot_weekday_bvu:BuiltPlot|undefined=undefined;
	let plot_weekday_rvu:BuiltPlot|undefined=undefined;

	get_plot_data("data/rotation_by_weekday"+key+".json");

	$:{
		if(data!==undefined)
		{
			plot_weekday_bvu=build_plot(data,container1_width,"bvu","BVU");
			plot_weekday_rvu=build_plot(data,container1_width,"rvu","RVU");
		}
	}
	
	let container1_width:any=undefined;
	$:{console.debug("Should be able to set plot width with container1_width",container1_width);}
</script>

<div class="container1" bind:clientWidth={container1_width}>
	Analysis
	{#if plot_weekday_bvu!==undefined}
		<ObservablePlot plot={plot_weekday_bvu}/>
	{/if}
	{#if plot_weekday_rvu!==undefined}
		<ObservablePlot plot={plot_weekday_rvu}/>
	{/if}
</div>

<style>
	.container1 {
		display: flex;
		flex-direction:column;
		height: 100%;
		overflow-y: auto;
	}
</style>