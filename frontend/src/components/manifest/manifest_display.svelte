<script lang="ts">
	import Drawer, { AppContent, Content } from '@smui/drawer';
	import Select, { Option } from '@smui/select';
  	import List, { Item, Text } from '@smui/list';
	import Button, { Group, Label } from '@smui/button';
	import type { Rotation, RotationManifest } from "./RotationManifest";
	import RotationDisplay from './rotation_display.svelte';
	import { day_indices, dowfunc, shortdowfunc, shortdowfuncinv } from '../../commons/time';
	import { onMount } from 'svelte';
	import DrawerToggleButton from '../common/drawer_toggle_button.svelte';
	import { key } from '../../commons/key';
	import Switch from '@smui/switch';
  	import FormField from '@smui/form-field';

	let manifest:RotationManifest|undefined=undefined;
	export let proposed:boolean;

	let fetch_string:string;
	$:{
		if(proposed)
		{
			fetch_string="data/active_rotation_manifest"+key+".json";
		}
		else
		{
			fetch_string="data/proposed_rotation_manifest"+key+".json";
		}
	}

	
	onMount(() => {
		fetch(fetch_string).then(
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
	let shortdow:string;
	$:{
		shortdow = shortdowfunc(dow);
	}

	let open=true;

	let display_warning:boolean=false;

	$ : {
		display_warning=dow!=today;
	}

	let empty_day:boolean=true;
	let responsible_days_arr:number[]=[];
	$:{
		empty_day=true;
		let responsible_days = new Set<number>();
		if(selected_rotation && selected_rotation.responsibilities)
		{
			for(const responsibility of selected_rotation.responsibilities)
			{
				for(const day of responsibility.days)
				{
					const di=shortdowfuncinv(day);
					if(di===undefined){console.error("Unexpected short day",day);}
					else
					{
						responsible_days.add(di);
						if(di === dow)
						{
							empty_day=false;
						}
					}
				}
			}	
		}
		responsible_days_arr=[];
		responsible_days.forEach((v)=>{responsible_days_arr.push(v);})
		responsible_days_arr.sort();
	}

	let show_wetreads:boolean=false;
	let show_volumes:boolean=false;
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
		<div class="manifest">
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
					<div class="rotation_header">
							<div class="rotation_label">{selected_rotation.rotation} {dowfunc(dow)}</div>
							<FormField>
								<Switch bind:checked={show_wetreads} />
								<span slot="label">Show Wet Reads</span>
							</FormField>
							<FormField>
								<Switch bind:checked={show_volumes} />
								<span slot="label">Show Responsibility Volumes</span>
							</FormField>
					</div>
					<RotationDisplay rotation={selected_rotation} dow={dow} hide_wetreads={!show_wetreads} hide_volumes={!show_volumes}/>
					{#if empty_day}
						<div class="warning warning_empty">
							<div class="warning_empty_desc">
								Warning: This rotation has no responsibilities on {shortdow}.
								Try:
							</div>
							<div class="warning_empty_buttons">
								{#each responsible_days_arr as day}
									<Button color="primary" variant="raised" style="margin-left:15px;" on:click={()=>{dow=day}}>
										<Label>{shortdowfunc(day)}</Label>
									</Button>
								{/each}
							</div>
						</div>
					{/if}
				{/if}
				{#if display_warning}
					<div class="warning warning_empty">
						<div class="warning_empty_desc">
							Warning: Today is {dowfunc(today)}, but {dowfunc(dow)} is selected.
						</div>
						<div class="warning_empty_buttons">
							<Button color="primary" variant="raised" on:click={()=>{dow=today}}>
								<Label>Go To Today</Label>
							</Button>
						</div>
					</div>
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
		font-size: larger;
		justify-content: space-between;
		align-content: center;
		align-items: center;
		border-top: 1px solid;
		border-bottom: 1px solid;
		padding-left: 5px;
		flex-shrink: 1;
	}

	.rotation_label{
		flex-shrink: 0;
		flex-grow:1;
		text-align: left;
		text-justify: center;
		font-size: larger;
		font-weight: bold;
		height: 100%;
	}

	.rotation_header{
		display:flex;
		justify-content:space-between;
		flex-direction: row;
	}

	.warning_empty {
		display:flex;
		justify-content: center;
		flex-direction: column;
	}
	.warning_empty_desc {
		flex-shrink:1;
		text-align: center;
	}
	.warning_empty_buttons {
		display: flex;
		flex-direction: row;
		justify-content:center;
	}

	.manifest {
		display: flex;
		flex-direction:column;
		height:100%;
		width:100%;
		flex-grow:1;
		flex-shrink:1;
		overflow:auto;
	}
</style>
