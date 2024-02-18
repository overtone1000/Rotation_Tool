<script lang="ts">
	import { key } from "../../commons/key";
	import ObservablePlot from "./observable_plot.svelte";
	import * as Plot from "@observablehq/plot";

	let data:any;
	const get_data=()=>{
		fetch("data/week_analysis"+key+".json").then(
			(value:Response)=>{
				if(value.ok)
				{
					value.json().then(
						(res:any)=>{
							data=res;
							console.debug("data is now ",data);
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

	let plot1:any=undefined;
	$ : {

		let rows=[];
		for(let rotation_name in data)
		{
			let rotation_data=data[rotation_name];
			for(let day_of_week in rotation_data)
			{
				let day_data=rotation_data[day_of_week];
				
			}
		}

		let marks=[];
		marks.push(Plot.frame());
		for (let row of data)
		{
			marks.push(row,)
		}

		plot1=Plot.plot({
		marks: [
			Plot.frame(),
			Plot.lineY(test_data, {x:"Date",y:"Close"}),
		]
	})
	}

	get_data();
</script>

<div class="container1">
	Analysis
	<ObservablePlot plot={plot1}/>
</div>

<style>
	.container1 {
		display: flex;
		flex-direction:column;
		height: 100%;
	}
</style>