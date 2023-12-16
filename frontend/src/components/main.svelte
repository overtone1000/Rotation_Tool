<script lang="ts">
	import Radio from '@smui/radio';
  	import FormField from '@smui/form-field';
	import CoverageDisplay from './coverage/coverage_display.svelte';
	import ManifestDisplay from './manifest/manifest_display.svelte';
	import Switch from '@smui/switch';
	import { onMount } from 'svelte';
	import { browser } from '$app/environment';

	enum Display
	{
		Manifest,
		Coverage
	}	

	let current_display=Display.Manifest;

	const STORAGE_KEY = 'theme';
	const THEMES = {
		DARK: 'dark',
		LIGHT: 'light',
	};
	const DARK_PREFERENCE = '(prefers-color-scheme: dark)';
	
	let mounted:boolean=false;
	let dark_mode:boolean=false;

	onMount(()=>{
		console.debug(window);
		mounted=true;
		dark_mode=window.matchMedia(DARK_PREFERENCE).matches;
	});

	$ : {
		console.debug("Reactive element called.",mounted,dark_mode);
		if(mounted)
		{
			if (dark_mode) {
				document.body.classList.remove(THEMES.LIGHT);
				document.body.classList.add(THEMES.DARK);
			} else {
				document.body.classList.remove(THEMES.DARK);
      			document.body.classList.add(THEMES.LIGHT);
			}
		}
	}

	//<Switch color="primary" bind:checked={dark_mode} />
	
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
		<div class="spacer"></div>
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
	.top_menu
	{
		display: flex;
		flex-direction: row;
	}
	.page {
		display: flex;
		flex-direction: column;
		flex-grow: 1;
		min-height: 1px;
	}
	.spacer {
		flex-grow: 1;
	}
</style>
