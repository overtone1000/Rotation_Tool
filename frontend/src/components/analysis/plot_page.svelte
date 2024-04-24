<script lang="ts">
	import { key } from "../../commons/key";
	import ManagedPlot from "./plot_manager.svelte";
	import { build_site_plot, get_facility_marks } from "./facility_volumes";
	import { build_heatmap, get_rotation_marks } from "./rotation_heatmap";
	import type { ValueType } from "./commons";

	let valuetype:ValueType="rvu";
</script>

<div class="container">
    <div class="plot_container">
		<ManagedPlot 
			filename={"data/volume_by_date_and_rotation"+key+".json"}
			plot_options={{valuetype:valuetype, title:"Rotation Volumes"}}
			get_marks={get_rotation_marks}
			build_plot={build_heatmap}
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