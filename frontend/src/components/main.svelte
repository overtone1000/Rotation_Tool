<script lang="ts">
	import Radio from '@smui/radio';
  	import FormField from '@smui/form-field';
	import CoverageDisplay from './coverage/coverage_display.svelte';
	import ManifestDisplay from './manifest/manifest_display.svelte';
	
	enum Display
	{
		Manifest,
		Coverage
	}	

	let current_display=Display.Manifest;
</script>

<div class="vp_fill">
	<div class="top_menu">
		<FormField>
			<Radio bind:group={current_display} value={Display.Manifest} touch />
			<span slot="label">Rotation Manifest</span>
		</FormField>
		<FormField>
			<Radio bind:group={current_display} value={Display.Coverage} touch />
			<span slot="label">Coverage Query</span>
		</FormField>
	</div>
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
		display:flex;
		flex-direction:column;
	}
	.page {
		display: flex;
		flex-direction: column;
		flex-grow: 1;
		min-height: 1px;
	}
</style>
