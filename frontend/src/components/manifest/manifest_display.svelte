<script lang="ts">
	import Drawer, { AppContent, Content } from '@smui/drawer';
	import Select, { Option } from '@smui/select';
  	import List, { Item, Text } from '@smui/list';
	import type { Rotation, RotationManifest } from "./RotationManifest";
	import RotationDisplay from './rotation_display.svelte';
	import { day_indices, dowfunc } from '../../commons/time';
	import { onMount } from 'svelte';

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
</script>

{#if manifest !== undefined}
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
		<div class="container2">
			<div class="drawer">
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
			<div class="manifest">
				{#if selected_rotation!==undefined}
					<RotationDisplay rotation={selected_rotation} dow={dow}/>
				{/if}
				{#if dow!==today}
					<div class="title" style="background:#5e0606; color:white">
						<Text>
							Warning: Today is {dowfunc(today)}, but {dowfunc(dow)} is selected.
						</Text>
					</div>
				{/if}
			</div>
		</div>
	</div>
{/if}

<style>
	.container1 {
		display: flex;
		flex-direction:column;
		height: 100%;
		min-height: 1px;
		flex-shrink: 1;
	}
	.container2{
		display: flex;
		flex-direction:row;
		min-height: 1px;
		flex-grow: 1;
		flex-shrink: 1;
	}
	.title {
		display: flex;
		justify-content: center;
		align-content: center;
		border-bottom-style: solid;
		border-bottom-width: 1px;
	}
	.manifest {
		display: flex;
		flex-direction:column;
		flex-grow: 1;
		flex-shrink: 1;
		min-height: 1px;
	}
	.drawer {
		flex-shrink: 1;
	}
</style>
