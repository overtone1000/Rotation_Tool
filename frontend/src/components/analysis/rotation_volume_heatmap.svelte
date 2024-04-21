<script lang="ts">
	import { key } from "../../commons/key";
	import type { AnalysisData, AnalysisMark } from "../../commons/plot_data";
	import { short_days_of_the_week_Mon_first } from "../../commons/time";
	import ObservablePlot from "./observable_plot.svelte";
	import * as Plot from "@observablehq/plot";
	import { workaround, workaround_tips } from "./workaround";

	type BuiltPlot = (SVGSVGElement | HTMLElement) & Plot.Plot;

	let data:AnalysisData|undefined=undefined;
	const get_plot_data=(file:string)=>{
		fetch(file).then(
			(value:Response)=>{
				if(value.ok)
				{
					value.json().then(
						(res:AnalysisData)=>{
							data=res;
							console.debug(data);
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

    let valuetype:"bvu"|"rvu"="rvu";
	const rotation_filter = (d:AnalysisMark) => {
		return d.rotation==="BR/US";
	}

	const dow_filter = (d:string) => {
		return d==="Mon";
	}

	const get_all_marks=(data:AnalysisData)=>{
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

	const build_weekday_plot=(data:AnalysisData, width:number, y:string, title:string)=>{	
		//console.debug("Building plot with",data,data.series);
		let displayed_marks=get_all_marks(data).filter(rotation_filter);

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

	const build_heatmap=(data:AnalysisData, width:number, y:string, title:string)=>{
		if (displayed_marks!==undefined)
		{
			let max=0;
			let min=0;
			for(let mark of displayed_marks)
			{
				if(mark.value>max){max=mark.value;}
				if(mark.value<min){min=mark.value;}
			}

			let outputs:Plot.BinOutputs={};

			let marks:any=[
				Plot.rect(
					displayed_marks,
					Plot.binY(
						{
							fill:"count",
							members:"identity"
						},
						workaround,
					)
				),
				Plot.tip(
					displayed_marks,
					Plot.binY(
						{
							fill:"count",
						},
						workaround_tips//Plot.pointer(workaround_tips)
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
	}
	let plot_weekday_bvu:BuiltPlot|undefined=undefined;
	let plot_weekday_rvu:BuiltPlot|undefined=undefined;
	let plot_heatmap_rvu:BuiltPlot|undefined=undefined;

	let selected_plot:BuiltPlot|undefined=undefined;

	get_plot_data("data/volume_by_date_and_rotation"+key+".json");

    let container_width:any="5000px";

	let displayed_marks:AnalysisMark[]|undefined=undefined;

	$:{
		if(data!==undefined)
		{

			//selected_plot=build_weekday_plot(data,container_width,valuetype,"Rotation Volume by Weekday");
			displayed_marks=get_all_marks(data);
			selected_plot=build_heatmap(data,container_width,valuetype,"Rotation Volumes");
		}
	}
</script>

<div class="container">
    <div bind:clientWidth={container_width}>
        {#if selected_plot!==undefined}
            <ObservablePlot plot={selected_plot}/>
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