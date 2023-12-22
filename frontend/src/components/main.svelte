<script lang="ts">
	import Menu, { SelectionGroup, SelectionGroupIcon } from '@smui/menu';
  	import List, { Item, Text } from '@smui/list';
	import CoverageDisplay from './coverage/coverage_display.svelte';
	import ManifestDisplay from './manifest/manifest_display.svelte';
	import IconButton, { Icon } from '@smui/icon-button';

	
	enum Display
	{
		Manifest,
		Coverage
	}

	const display_to_string = (display:Display) => {
		switch(display)
		{
			case Display.Manifest:return "Rotation Descriptions";
			case Display.Coverage:return "Coverage Query";
		}
	}

	const light_css = "/smui.css";
	const dark_css = "/smui-dark.css";

	let current_display=Display.Manifest;

	let dark_mode:boolean|undefined=undefined;
	let menu: Menu;
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
		<div class="top_menu_item">
			<IconButton on:click={() => menu.setOpen(true)}>
				<Icon class="material-icons">menu</Icon>
			</IconButton>
			<Menu bind:this={menu}>
				<List>
					<SelectionGroup>
						{#each [Display.Manifest, Display.Coverage] as display_option}
						<Item
							on:SMUI:action={() => {
								current_display = display_option;
								menu.setOpen(false);
							}}
							selected={current_display === display_option}
						>
							<SelectionGroupIcon>
							<i class="material-icons">check</i>
							</SelectionGroupIcon>
							<Text>{display_to_string(display_option)}</Text>
						</Item>
						{/each}
					</SelectionGroup>
				</List>
			</Menu>
		</div>
		<div class="spacer"></div>
		<div class="top_menu_item">
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
	.top_menu_item
	{
		display:flex;
		align-items:center;
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
