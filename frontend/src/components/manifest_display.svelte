<script lang="ts">
	import Drawer, { AppContent, Content } from '@smui/drawer';
	import Select, { Option } from '@smui/select';
  	import List, { Item, Text } from '@smui/list';
	import type { RotationManifest } from "./RotationManifest";

	export let manifest:RotationManifest;
	console.debug("Manifest",manifest);

	let rotation = "";
	
	const today=(new Date()).getDay()
	let dow = today;

	const day_indices = [0,1,2,3,4,5,6];

	const days_of_the_week = [
		"Sunday",
		"Monday",
		"Tuesday",
		"Wednesday",
		"Thursday",
		"Friday",
		"Saturday"
	];

	const dowfunc=(di:number)=>{
		return days_of_the_week[di];
	}
</script>

<div class="container1">
	<div class="title">
		<Text class="title" style="align-self:center; margin-right:20px">{manifest.title}</Text>
		<div class="columns margins">
			<div>
			  <Select 
				key={dowfunc}
				bind:value={dow}
				>
				{#each day_indices as di}
				  <Option value={di}>{dowfunc(di)}</Option>
				{/each}
			  </Select>
			</div>
		</div>
	</div>
	{#if dow!==today}
		<div class="title" style="background:#5e0606; color:white">
			<Text>
				Warning: Today is {dowfunc(today)}, but {dowfunc(dow)} is selected.
			</Text>
		</div>
	{/if}
	<div class="container2">
		<div class="drawer">
			<Drawer>
				<Content>
					<List>
						{#each manifest.rotation_manifest as rotation}
							<Item>
								<Text>{rotation.rotation}</Text>
							</Item>
						{/each}
					</List>
				</Content>
			</Drawer>
		</div>		
		<div class="manifest">
			Manifest
		</div>
	</div>
</div>

<style>
	.container1 {
		display: flex;
		flex-direction:column;
		height: 100%;
	}
	.container2{
		display: flex;
		flex-direction:row;
		flex-grow: 1;
	}
	.title {
		display: flex;
		justify-content: center;
		align-content: center;
		border-bottom-style: solid;
		border-bottom-width: 1px;
	}
	.manifest {
		
	}
	.drawer {
		height: 100%;
		flex-shrink: 1;
	}
</style>
