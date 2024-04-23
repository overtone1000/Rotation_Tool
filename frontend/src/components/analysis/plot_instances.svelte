<script lang="ts">
	import { key } from "../../commons/key";
	import type { Rotation_Analysis_Data, AnalysisMark } from "../../commons/rotation_plot_data";
	import { short_days_of_the_week_Mon_first } from "../../commons/time";
	import ObservablePlot from "./observable_plot.svelte";
	import * as Plot from "@observablehq/plot";
	import { workaround, workaround_tips } from "./workaround";
	import type { FacilityAnalysisMark, Facility_Analysis_Data } from "../../commons/facility_plot_data";

	type BuiltPlot = (SVGSVGElement | HTMLElement) & Plot.Plot;

	let rotation_analysis_data:Rotation_Analysis_Data|undefined=undefined;
	let facility_analysis_data:Facility_Analysis_Data|undefined=undefined;

	const get_data=(file:string, callback:(raw_data_result:any)=>void)=>{
		fetch(file).then(
			(value:Response)=>{
				if(value.ok)
				{
					value.json().then(
						(res:any)=>{
							callback(res);
							console.debug(rotation_analysis_data);
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

	const get_rotation_marks=(data:Rotation_Analysis_Data)=>{
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
						value:rotation_data[valuetype]
					}
				);
			}
		}
		
		return all_marks;		
	}

	const get_facility_marks=(data:Facility_Analysis_Data)=>{
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
						value:facility_data[valuetype]
					}
				);
			}
		}
		
		return all_marks;		
	}

	const build_weekday_plot=(data:Rotation_Analysis_Data, width:number, y:string, title:string)=>{	
		//console.debug("Building plot with",data,data.series);
		let displayed_marks=get_rotation_marks(data).filter(rotation_filter);

		let marks:any=[
			Plot.frame(),
			Plot.barY(
				displayed_marks.filter(rotation_filter),
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

	const build_heatmap=(analysis_marks:AnalysisMark[], width:number, y:string, title:string)=>{
		let max=0;
		let min=0;
		for(let mark of analysis_marks)
		{
			if(mark.value>max){max=mark.value;}
			if(mark.value<min){min=mark.value;}
		}

		let outputs:Plot.BinOutputs={};

		let marks:any=[
			Plot.rect(
				analysis_marks,
				Plot.binY(
					{
						fill:"proportion-facet",
						members:"identity"
					},
					workaround,
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
			title: title,
			color: {legend:true},
			width: width,
			//aspectRatio: 1,
			height: 600,
			y:{
				grid:false,
				label:valuetype
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

	const build_site_plot=(facility_marks:FacilityAnalysisMark[], width:number, y:string, title:string)=>{

		let marks:any=[
			Plot.barY(
				facility_marks,
				Plot.groupX(
					{
						y:"sum"
					},
					{
						y:"value",
						x:"date",
						fill:"facility",
						order:"sum",
						reverse:false
					}
				)
			),
		];

		const retval = Plot.plot({
			title: title,
			color: {legend:true},
			width: width,
			//aspectRatio: 1,
			height: 600,
			y:{
				grid:false,
				label:valuetype
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

	let rotation_plot:BuiltPlot|undefined=undefined;
	let facility_plot:BuiltPlot|undefined=undefined;

	get_data("data/volume_by_date_and_rotation"+key+".json",(result:any)=>{rotation_analysis_data=result as Rotation_Analysis_Data});
	get_data("data/volume_by_date_and_facility"+key+".json",(result:any)=>{facility_analysis_data=result as Facility_Analysis_Data});

    let container_width:any="5000px";

	let displayed_rotation_marks:AnalysisMark[]|undefined=undefined;
	let displayed_facility_marks:FacilityAnalysisMark[]|undefined=undefined;

	$:{
		if(rotation_analysis_data!==undefined)
		{
			displayed_rotation_marks=get_rotation_marks(rotation_analysis_data);
		}
	}
	$:{
		if(displayed_rotation_marks!==undefined)
		{
			rotation_plot=build_heatmap(displayed_rotation_marks,container_width,valuetype,"Rotation Volumes");
		}
	}

	$:{
		if(facility_analysis_data!==undefined)
		{
			displayed_facility_marks=get_facility_marks(facility_analysis_data);
		}
	}
	$:{
		if(displayed_facility_marks!==undefined)
		{			
			facility_plot=build_site_plot(displayed_facility_marks,container_width,valuetype,"Facility Volumes");
		}
	}
</script>

<div class="container">
    <div class="plot_container" bind:clientWidth={container_width}>
		{#if facility_plot!==undefined}
			<ObservablePlot plot={facility_plot}/>
		{/if}
        {#if rotation_plot!==undefined}
            <ObservablePlot plot={rotation_plot}/>
        {/if}
    </div>
    <div class="controls">
        Hello
    </div>
</div>

<style>
    .container {
		display: flex;
		flex-direction:row;
        width:100%;
	}
	.plot_container {
		width:100%;
	}
    .controls {
        display: flex;
        flex-direction: column;
        justify-content: space-evenly;
    }
</style>