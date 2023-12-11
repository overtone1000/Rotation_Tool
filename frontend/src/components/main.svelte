<script lang="ts">
	import { onMount } from 'svelte';
	import ManifestDisplay from './manifest_display.svelte';
	import type { RotationManifest } from "./RotationManifest";

	let manifest_instance:RotationManifest|undefined=undefined;
	
	onMount(() => {
		fetch("active.json").then(
			(value:Response)=>{
				if(value.ok)
				{
					value.json().then(
						(manifest:RotationManifest)=>{
							manifest_instance=manifest;
						}
					);
				}
			}
		);
	});


</script>

<div class="vp_fill">
	<div class="page">
		{#if manifest_instance!==undefined}
			<ManifestDisplay manifest={manifest_instance}/>
		{/if}
	</div>
</div>

<style>
	.vp_fill {
		width: 100vw;
		height: 100vh;
		max-width: 100vw;
		max-height: 100vh;
		overflow: hidden;
	}
	.page {
		width: 100%;
		height: 100%;
		display: flex;
		flex-direction: column;
	}
</style>
