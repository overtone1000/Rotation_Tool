<script lang="ts">
	import Radio from '@smui/radio';
  	import FormField from '@smui/form-field';
	import CoverageDisplay from './coverage/coverage_display.svelte';
	import ManifestDisplay from './manifest/manifest_display.svelte';
	import IconButton, { Icon } from '@smui/icon-button';
	import { onMount } from 'svelte';
	
	enum Display
	{
		Manifest,
		Coverage
	}

	const light_css = "/smui.css";
	const dark_css = "/smui-dark.css";

	let current_display=Display.Manifest;

	let dark_mode:boolean=false;
	$:{console.debug("Darkmode is now",dark_mode);}
</script>


<svelte:head>
  {#if dark_mode === undefined}
  <link
    rel="stylesheet"
    href={light_css}
    media="(prefers-color-scheme: light)"
  />
  <link
    rel="stylesheet"
    href={dark_css}
    media="screen and (prefers-color-scheme: dark)"
  />
  {:else if dark_mode}
	<link rel="stylesheet" href={light_css} media="print" />
	<link rel="stylesheet" href={dark_css} media="screen" />
  {:else}
  	<link rel="stylesheet" href={light_css} />
  {/if}
</svelte:head>

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
		<div style="display: flex; align-items: center;">
			<IconButton on:click={() => dark_mode=!dark_mode} toggle pressed={dark_mode}>
				<Icon class="material-icons" on>light_mode</Icon>
				<Icon class="material-icons">dark_mode</Icon>
			</IconButton>
		</div>
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
