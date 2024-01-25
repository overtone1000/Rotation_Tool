<script lang="ts">
	import { shortdowfunc, time_range_to_string } from "../../commons/time";
	import type { Responsibility, Rotation, WorkHoursPeriod } from "./RotationManifest";
	import ResponsibilityDisplay from './responsibility_display.svelte';

	export let rotation:Rotation;
	export let dow:number;
	export let hide_wetreads:boolean;

	let shortdow:string;
	$:{
		shortdow = shortdowfunc(dow);
	}

	const should_display = (responsibility:Responsibility) => {
		let retval = responsibility.days=="All" || responsibility.days.includes(shortdow);
		return retval;
	}

	let pertinent_rotation_hours:WorkHoursPeriod|undefined=undefined;
	$:{
		pertinent_rotation_hours=undefined;
		if(rotation.hours)
		{
			for(const entry of rotation.hours)
			{
				for(const day of entry.days)
				{
					if(shortdow==day)
					{
						if(pertinent_rotation_hours!=undefined)
						{console.error("Duplicate rotation hour days.",rotation.hours)}
						pertinent_rotation_hours=entry;
					}
				}
			}
		}
	}
</script>

<div class="container">
	{#if rotation.responsibilities}
		<table class="table">
			<tr>
				<th>Site</th>
				<th>Context</th>
				<th>Exams</th>
				<th>Portions</th>
			</tr>
			{#key dow}
				{#key rotation.responsibilities}
					{#if rotation.responsibilities!==undefined && rotation.responsibilities!==null}
						{#each rotation.responsibilities as responsibility}
							{#if should_display(responsibility)}
								<ResponsibilityDisplay
									responsibility={responsibility}
									dow={dow}
									hide_wetreads={hide_wetreads}
								/>
							{/if}
						{/each}
					{/if}
				{/key}
			{/key}
		</table>
	{/if}
	<div>
		{#if rotation.location}
			<div>
				Location: {rotation.location}
			</div>
		{/if}
		{#if pertinent_rotation_hours}
		<div>
			Hours: {time_range_to_string(pertinent_rotation_hours.hours,dow)}
		</div>
		{/if}
		{#if rotation.breaktime}
			<div>
				Break: {time_range_to_string(rotation.breaktime[0],dow)}
			</div>
			{#if rotation.breaktime[1]}
				<div style="padding-left: 20px">
					{rotation.breaktime[1]}
				</div>
			{/if}
		{/if}
		{#if rotation.comments !== undefined && rotation.comments !== null}
		<ul>
			{#each rotation.comments as comment}
				<li><div class="mdc-typography--body1">{comment}</div></li>
			{/each}
		</ul>
	{/if}
	</div>
</div>
<style>
	.container {
		display: flex;
		flex-direction:column;
		height:100%;
		width:100%;
		flex-grow:1;
		flex-shrink:1;
		overflow:auto;
	}
	table, th {
		border: 1px solid;
		border-collapse: collapse;
	}
</style>
