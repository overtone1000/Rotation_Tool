<script lang="ts">
	import Drawer, { AppContent, Content } from '@smui/drawer';
	import Select, { Option } from '@smui/select';
  	import List, { Item, Text } from '@smui/list';
	import type { Rotation, RotationManifest } from "./RotationManifest";
	import RotationDisplay from './rotation_display.svelte';
	import { day_indices, dowfunc } from '../../commons/time';
	import { onMount } from 'svelte';
	import DrawerToggleButton from '../common/drawer_toggle_button.svelte';
	
	let manifest:RotationManifest|undefined=undefined;
	
	onMount(() => {
		fetch("active_rotation_manifest.json").then(
			(value:Response)=>{
				if(value.ok)
				{
					value.json().then(
						(res:RotationManifest)=>{
							manifest=res;
						}
					);
				}
			}
		);
	});

	let selected_rotation:Rotation|undefined = undefined;
	
	const today=(new Date()).getDay()
	let dow = today;

	let open=true;

	const warning_background="#320303";
	const black_background="black";
	
	let display_warning:boolean=false;
	let background=black_background;

	$ : {
		display_warning=dow!=today;

		console.debug("Display warning now ",display_warning);
		if(display_warning)
		{
			background=warning_background;
		}
		else
		{
			background=black_background;
		}
	}
</script>

{#if manifest !== undefined}
	<div class="container1">
		<div class="drawer" hidden={!open}>
			<Drawer>
				<Content>
					<List>
						{#each manifest.rotation_manifest as rotation}
							<Item
							on:click={()=>{selected_rotation=rotation}}
							>
								<Text>{rotation.rotation}</Text>
							</Item>
						{/each}
					</List>
				</Content>
			</Drawer>		
		</div>
		<div class="manifest" style="background:{background};">
			<div class="title">
				<DrawerToggleButton bind:open={open}/>
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
			<div class="manifest">
				{#if selected_rotation!==undefined}
					<div class="rotation_label">{selected_rotation.rotation} {dowfunc(dow)}</div>
					<RotationDisplay rotation={selected_rotation} dow={dow}/>
				{/if}
				{#if display_warning}
					<p class="warning">
						Warning: Today is {dowfunc(today)}, but {dowfunc(dow)} is selected.
					</p>
				{/if}
			</div>
		</div>
	</div>
{/if}

<style>
	.container1 {
		display: flex;
		flex-direction:row;
		height: 100%;
	}
	.title {
		display: flex;
		justify-content: space-between;
		align-content: center;
		border-top: 1px solid white;
		border-bottom: 1px solid white;
		padding-left: 5px;
		flex-shrink: 1;
	}
	.rotation_label{
		flex-shrink: 0;
		text-align: center;
	}
	.warning {
		display: flex;
		justify-content: center;
		align-content: center;
		text-align:left;
		border-top: 1px solid white;
		border-bottom: 1px solid white;
		padding-left: 5px;
		background:#5e0606;
		color:white;
		flex-grow:1;
		flex-shrink:1;
		margin: 1px;
		padding: 1px;
	}
	.manifest {
		display: flex;
		flex-direction:column;
		height:100%;
		width:100%;
		flex-grow:1;
		flex-shrink:1;
		overflow:scroll;
	}
</style>
