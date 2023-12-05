<script lang="ts">
	import Button from '@smui/button';
	import { Label } from '@smui/tab';
	import { StagingTypes } from '../../commons/refactored/staging/data_processing/stagingdata';
	import { StagingSelectionMode } from '../../commons/refactored/staging/members/highlighting';
	import { GenericRenderedConstraint } from '../../commons/refactored/staging/members/rendered_constraint';
	import { stagingProcessedMembers, stagingSelection, type Selectable } from '../stores';

	import IconButton from '@smui/icon-button';
	import { Staging_Directives, type ASRequestStagingParameters } from '../../commons/refactored/ajax/commands_generic';
	import { RenderedAssignable } from '../../commons/refactored/staging/members/rendered_assignable';
	import { stagingModification } from '../staging';
	import ConstraintEditor from './constraint_editor.svelte';
	import WorkerSelection from './worker_selection.svelte';

	const enum DisplayMode {
		Empty,
		SingleAssignable,
		MultipleAssignables,
		SingleConstraint,
		Mixed
	}

	let display_mode = DisplayMode.Empty;
	let single_selection: Selectable | undefined = undefined;

	const getDisplayMode = (member: Selectable, current_display_mode:DisplayMode) => {
		switch (member.getStagingType()) {
			case StagingTypes.Assignable:
				{
					switch (current_display_mode) {
						case DisplayMode.Empty:
							return DisplayMode.SingleAssignable;
						case DisplayMode.SingleAssignable:
						case DisplayMode.MultipleAssignables:
							return DisplayMode.MultipleAssignables;
						case DisplayMode.SingleConstraint:
							return DisplayMode.Mixed;
						default:
							console.debug("Default",member,member.getStagingType(),current_display_mode);
							return DisplayMode.Empty;
					}
				}
			case StagingTypes.Constraint:
				{
					switch (display_mode) {
						case DisplayMode.Empty:
							return DisplayMode.SingleConstraint;
						default:
							return DisplayMode.Mixed;
					}
				}
			default:
				console.debug("Overall default",member,current_display_mode);
				return DisplayMode.Empty;
		}
	};

	//Reacts to staging selection change
	let primary_selections:Set<Selectable>|undefined;
	$: {		
		primary_selections = $stagingSelection.selections.get(
			StagingSelectionMode.primary_selected
		);
		console.debug("Staging selection change!",primary_selections);
		//Sets correct display mode
		display_mode = DisplayMode.Empty;
		if(primary_selections)
		{
			for(const selection of primary_selections.values())
			{
				display_mode=getDisplayMode(selection,display_mode);
			}
		}

		if (primary_selections?.size == 1) {
			primary_selections?.forEach((selection) => {
				single_selection = selection;
			});
		} else {
			single_selection = undefined;
		}
	}

	const getSelectedIndices = () => {
		const ids=[] as number[];
		if(primary_selections)
		{
			for(const selectable of primary_selections)
			{
				const id=(selectable as RenderedAssignable).getIndex();
				if(id!==undefined)
				{
					ids.push(id);
				}
			}
		}
		return ids;
	};

	const changeLockedState = (locked:boolean) => {
		const parameters={} as ASRequestStagingParameters;
		if(primary_selections && primary_selections.size>0)
		{
			if(locked)
			{
				parameters.type=Staging_Directives.lock;
			}
			else
			{
				parameters.type=Staging_Directives.unlock;
			}

			parameters.staging_ids=getSelectedIndices();
			stagingModification(parameters);
		}
	}

	const unassign = () => {
		const parameters={} as ASRequestStagingParameters;
		parameters.type=Staging_Directives.unassign;
		parameters.staging_ids=getSelectedIndices();
		stagingModification(parameters);
	}

	const save_constraint = () => {
		const constraint = single_selection as GenericRenderedConstraint;
		if($stagingProcessedMembers)
		{
			constraint.sendModification($stagingProcessedMembers);
		}
	}

</script>

<div class="container">
	{#if display_mode === DisplayMode.SingleAssignable && single_selection && single_selection instanceof RenderedAssignable}
		<WorkerSelection
			ra={single_selection}
		/>
	{/if}
	{#if display_mode === DisplayMode.SingleAssignable || display_mode === DisplayMode.MultipleAssignables}
		<div class="row">
		<IconButton class="material-icons" on:click={() => {changeLockedState(true)}} 
			>lock</IconButton>
		<IconButton class="material-icons" on:click={() => {changeLockedState(false)}}
			>lock_open</IconButton>
		</div>
		<div class="button">
			<Button
				variant="raised"
				color="secondary"
				on:click={unassign}
			>
				<Label>Unassign</Label>
			</Button>
		</div>
	{/if}
	{#if display_mode === DisplayMode.SingleConstraint}
		{#if single_selection instanceof GenericRenderedConstraint}
			<ConstraintEditor constraint={single_selection} />
			<div class="button">
				<Button
					variant="raised"
					color="secondary"
					on:click={() => {
						save_constraint();
					}}
				>
					<Label>Save Changes</Label>
				</Button>
			</div>
		{/if}
	{/if}
	{#if display_mode === DisplayMode.Empty}
		<p>No members selected.</p>
	{:else}
		<div class="spacer" />
		<div class="button">
			<Button
				variant="raised"
				on:click={() => {
					console.debug('Delete');
				}}
			>
				<Label>Delete</Label>
			</Button>
		</div>
	{/if}
</div>

<style>
	.container {
		display: flex;
		flex-direction: column;
		box-sizing: border-box;
		height: 100%;
		padding-top:5px;
		justify-items:center;
		justify-content: center;
	}
	p {
		margin-top: 40px;
		text-align: center;
	}
	.spacer {
		display: flex;
		flex-grow: 1;
		flex-shrink: 1;
	}
	.row {
		display: flex;
		flex-direction: row;
		justify-content: space-evenly;
	}
</style>
