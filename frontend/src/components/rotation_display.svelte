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
		if(retval){
			console.debug("Showing",responsibility,shortdow);
		}

		return retval;
	}
</script>

<div>
	{#key shortdow}
		{#each rotation.responsibilities as responsibility}
			{#if should_display(responsibility)}
				<ResponsibilityDisplay
					responsibility={responsibility}
				/>
			{/if}
		{/each}
	{/key}
</div>

<style>

</style>
