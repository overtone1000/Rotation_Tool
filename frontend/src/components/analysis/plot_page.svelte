<script lang="ts">
	import { key } from "../../commons/key";
	import ManagedPlot from "./plot_manager.svelte";
	import { build_site_plot, get_facility_marks } from "./facility_volumes";
	import { build_heatmap, get_rotation_marks } from "./rotation_heatmap";
	import type { ValueType } from "./commons";
	import FormField from "@smui/form-field";
	import Radio from '@smui/radio';
	import Switch from "@smui/switch";
	import Button, { Icon, Label } from "@smui/button";
	import MultiSelect from 'svelte-multiselect'
	import type { Rotation_Analysis_Data } from "../../commons/rotation_plot_data";
	import { build_comparison, get_comparison_marks } from "./comparison";

	let valuetype:ValueType="bvu";
	let valuetypeswitch:boolean=false;
	$:{
		if(valuetypeswitch)
		{valuetype="bvu";}
		else
		{valuetype="rvu";}
	}

	let sourcetype:"active"|"proposed"="active";
	let sourcetypeswitch:boolean=true;
	$:{
		if(sourcetypeswitch)
		{sourcetype="active";}
		else
		{sourcetype="proposed";}
	}
	let heatmap_filename:string="";
	$:{
		if(sourcetype=="proposed")
		{
			heatmap_filename="data/volume_by_date_and_rotation_proposed"+key+".json"
		}
		else
		{
			heatmap_filename="data/volume_by_date_and_rotation_active"+key+".json"
		}
	}

	let rotation_list:string[]|undefined=undefined;
	let selected:string[]=[];

	const volume_data_callback = (volume_data:Rotation_Analysis_Data)=>{
		let rotation_set:Set<string>= new Set();

		console.debug("Rotation set update.");

		for(let date in volume_data.date_map)
		{
			let date_entry=volume_data.date_map[date];
			for(let rotation in date_entry)
			{
				rotation_set.add(rotation);
			}
		}
		if(rotation_set.size>0)
		{
			selected=[];
			rotation_list=[];
			for(let rotation of rotation_set)
			{
				selected.push(rotation);
				rotation_list.push(rotation);
			}
		}
		else
		{
			rotation_list=undefined;
		}
	}
</script>

<div class="container">
	<div class="controls">
		<div>
			<FormField>
				Value type:
				<Switch bind:checked={valuetypeswitch} color="secondary" icons={false} />
				<span slot="label">{valuetype}</span>
			</FormField>
		</div>
		<div>
			<FormField>
				Source:
				<Switch bind:checked={sourcetypeswitch} color="secondary" icons={false} />
				<span slot="label">{sourcetype}</span>
			</FormField>
		</div>
		<div class="multiselect">
			{#if rotation_list!==undefined}
				<MultiSelect bind:selected options={rotation_list}  --sms-li-bg="black" />
			{/if}
		</div>
	</div>
    <div class="plot_container">
		<ManagedPlot 
			filename={heatmap_filename}
			plot_options={{valuetype:valuetype, title:"Rotation Volumes", rotations:selected}}
			get_marks={get_rotation_marks}
			build_plot={build_heatmap}
			data_callback={volume_data_callback}
		/>
		<ManagedPlot 
			filename={"data/proposed_differential"+key+".json"}
			plot_options={{valuetype:valuetype, title:"Proposal Differentials", rotations:selected}}
			get_marks={get_comparison_marks}
			build_plot={build_comparison}
		/>
        <ManagedPlot 
			filename={"data/volume_by_date_and_facility"+key+".json"}
			plot_options={{valuetype:valuetype, title:"Facility Volumes"}}
			get_marks={get_facility_marks}
			build_plot={build_site_plot}
		/>
    </div>
</div>

<style>
    .container {
		display: flex;
		flex-direction:column;
        width:100%;
	}
	.plot_container {
		width:100%;
	}
    .controls {
        display: flex;
        flex-direction: row;
        justify-content: space-evenly;
    }
	.multiselect {
		width: 50%;
	}
	:global(div.multiselect > ul.options > li)
	{
		color: white;
		background-color: black;
	}
	:global(div.multiselect > ul.options > li:not(.selected):hover) {
		color: black;
		background-color: white;
	}
</style>