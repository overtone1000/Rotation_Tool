<script lang="ts">
	import Textfield from "@smui/textfield";
	import PlotInstances from "./plot_instances.svelte";

	let admitted:boolean;
	let text:string="";
	$:{
		admitted= text==="showmethestats";
	}
</script>

<div class="container">

	{#if admitted}
		<PlotInstances />
	{:else}
		<Textfield bind:value={text} label="Passcode"/>
	{/if}
	
</div>

<style>
	.container {
		display: flex;
		flex-direction:column;
		height: 100%;
		width: 100%;
		overflow-y: auto;
	}
</style>