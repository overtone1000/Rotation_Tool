<script lang="ts">
	import Panel from './panel/panel.svelte';
	import Table from './table/table.svelte';

	import { onMount } from 'svelte';

	import type { OperationContents } from '../commons/refactored/ajax/commands_generic';
	import { requestDisplayData } from './staging';
	import { stagingInit, stagingProcessedProposal, type StagingOperationContents } from './stores';

	onMount(() => {
		console.debug('Mounting.');
		requestDisplayData().then((response: OperationContents | undefined) => {
			if (response) {
				$stagingInit = response as StagingOperationContents;
			}
		});
	});
</script>

<div class="vp_fill">
	<div class="page">
		<!--<div class="top_menu">Menu</div>-->

		{#if $stagingProcessedProposal == undefined}
			<div>Awaiting response</div>
		{:else}
			<div class="display">
				<Table staging_data={$stagingProcessedProposal} />
				<Panel />
			</div>
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
		/* border: 5px;
        border-style: solid;
        border-color: white; */
	}
	.top_menu {
		width: 100%-1px;
		height: 90px;
		flex-shrink: 0;
		border: 1px;
		border-style: solid;
		border-color: blue;
	}
	.display {
		display: flex;
		flex-direction: row;
		flex-grow: 1;
		overflow-y: hidden;
	}
</style>
