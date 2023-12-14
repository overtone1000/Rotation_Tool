<script lang="ts">
	import CoverageDisplay from './coverage/coverage_display.svelte';
	import ManifestDisplay from './manifest/manifest_display.svelte';
	
	enum Display
	{
		Manifest,
		Coverage
	}	

	let current_display=Display.Coverage;
</script>

<div class="vp_fill">
	<div class="page">
		{#if current_display==Display.Manifest}
			<ManifestDisplay/>
		{:else if current_display==Display.Coverage}
			<CoverageDisplay/>
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
