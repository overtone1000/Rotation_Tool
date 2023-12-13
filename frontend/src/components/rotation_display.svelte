<script lang="ts">
	import { shortdowfunc } from "../commons/time";
	import type { Responsibility, Rotation } from "./RotationManifest";
	import ResponsibilityDisplay from './responsibility_display.svelte';

	export let rotation:Rotation;
	export let dow:number;
	let shortdow:string;
	$:{
		shortdow = shortdowfunc(dow);
		console.debug(shortdow);
	}

	const should_display = (responsibility:Responsibility) => {
		let retval = responsibility.days=="All" || responsibility.days.includes(shortdow);
		return retval;
	}
</script>

<div>
	<table class="table">
		<tr>
			<th>Site</th>
			<th>Subspecialty</th>
			<th>Context</th>
			<th>Modality</th>
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
							/>
						{/if}
					{/each}
				{/if}
			{/key}
		{/key}
	</table>
	{#if rotation.comments !== undefined && rotation.comments !== null}
		<ul>
			{#each rotation.comments as comment}
				<li><div class="mdc-typography--body1">{comment}</div></li>
			{/each}
		</ul>
	{/if}
</div>

<style>
	table, th {
		border: 1px solid white;
		border-collapse: collapse;
	}
</style>
