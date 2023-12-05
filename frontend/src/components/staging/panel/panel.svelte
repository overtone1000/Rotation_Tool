<script lang="ts">
	import Tab, { Icon, Label } from '@smui/tab';
	import TabBar from '@smui/tab-bar';
	import { StagingSelectionMode } from '../../commons/refactored/staging/members/highlighting';
	import { RightPanelContext } from '../../commons/refactored/staging/staging';
	import {
		stagingContext,
		stagingProcessedMembers,
		stagingSelection,
		type Selectable
	} from '../stores';
	import Add from './add.svelte';
	import Commit from './commit.svelte';
	import Edit from './edit.svelte';

	type TabEntry = {
		icon: string;
		label: string;
	};

	const tabs: TabEntry[] = [];
	tabs[RightPanelContext.add] = {
		icon: 'add_circle',
		label: 'Add'
	};
	tabs[RightPanelContext.edit] = {
		icon: 'edit',
		label: 'Edit'
	};
	tabs[RightPanelContext.commit] = {
		icon: 'commit',
		label: 'Commit'
	};

	const contexts: RightPanelContext[] = [
		RightPanelContext.add,
		RightPanelContext.edit,
		RightPanelContext.commit
	];

	let active_tab = tabs[$stagingContext]; //initialize to whatever the stagingContext store value is, but bind to TabBar (below)
	$: {
		//when active tab is changed, update the store
		$stagingContext = tabs.indexOf(active_tab);
		switch (tabs.indexOf(active_tab)) {
			case RightPanelContext.add:
				stagingSelection.setBaseMode({ mode: StagingSelectionMode.none, multi: false });
				break;
			case RightPanelContext.edit:
				stagingSelection.setBaseMode({ mode: StagingSelectionMode.primary_selected, multi: false });
				break;
			case RightPanelContext.commit:
				stagingSelection.setBaseMode({ mode: StagingSelectionMode.commit, multi: false });
				{
					const commitables = new Set<Selectable>();
					for (const commitable_member of $stagingProcessedMembers!.response.update_data
						.commitable) {
						commitables.add($stagingProcessedMembers!.staging_members[commitable_member]);
					}
					stagingSelection.setSelection(StagingSelectionMode.commit, commitables);
				}
				break;
		}
	}
</script>

<div class="panel">
	<TabBar {tabs} let:tab bind:active={active_tab}>
		<Tab {tab} stacked={true} indicatorSpanOnlyContent={true} tabIndicator$transition="fade">
			<Icon class="material-icons">{tab.icon}</Icon>
			<Label>{tab.label}</Label>
		</Tab>
	</TabBar>
	<div class="panel_contents">
		{#if $stagingContext === RightPanelContext.add}
			<Add />
		{:else if $stagingContext === RightPanelContext.edit}
			<Edit />
		{:else if $stagingContext === RightPanelContext.commit}
			<Commit />
		{:else}
			Unhandled context: {$stagingContext}
		{/if}
	</div>
</div>

<style>
	.panel {
		display: flex;
		flex-direction: column;
		flex-shrink: 0;
		width: 300px;
		border-left-width: 2px;
		border-left-style: solid;
		border-left-color: white;
	}
	.panel_contents {
		width: 100%-10px;
		flex-shrink: 1;
		flex-grow: 1;
		/*display: flex;
      flex-direction: column;*/
		overflow-y: auto;
		overflow-x: hidden;
		margin: 5px;
	}
</style>
