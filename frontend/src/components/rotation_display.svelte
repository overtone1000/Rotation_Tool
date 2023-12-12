<script lang="ts">
	import type { Rotation } from "./RotationManifest";
	import ResponsibilityDisplay from './responsibility_display.svelte';

	export let rotation:Rotation;
</script>

<div>
	{#each rotation.responsibilities as responsibility}
		<ResponsibilityDisplay
			responsibility={responsibility}
		/>
	{/each}
</div>

<style>

</style>
