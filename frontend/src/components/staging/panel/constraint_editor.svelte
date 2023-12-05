<script lang="ts">
	import Button from '@smui/button';
	import { Label } from '@smui/tab';
	import { onDestroy } from 'svelte';
	import { BinaryNode } from '../../commons/refactored/data/BinaryNode';
	import { ConstraintClass } from '../../commons/refactored/extended_types/bndata/Constraint';
	import {
		MatchOneStagingDetailsCandidatesToBNDataClone,
		StagingTypes,
		type MatchOne_StagingDetails
	} from '../../commons/refactored/staging/data_processing/stagingdata';
	import { StagingSelectionMode } from '../../commons/refactored/staging/members/highlighting';
	import type { RenderedAssignable } from '../../commons/refactored/staging/members/rendered_assignable';
	import type {
		GenericRenderedConstraint,
		RenderedConstraint_MatchOne,
		RenderedConstraint_SingleWorker
	} from '../../commons/refactored/staging/members/rendered_constraint';
	import BinaryNodeDialog from '../../tables/edit/binary_node_parent.svelte';
	import { stagingInit, stagingProcessedMembers, stagingSelection } from '../stores';
	import ConstraintMemberSelector from './constraint_member_selector.svelte';

	export let constraint: GenericRenderedConstraint;
	
	let members_1: RenderedAssignable[];
	let members_2: RenderedAssignable[];
	let selected = false;

	const populateMembers = () => {
		if (constraint.getConstraintClass() == ConstraintClass.SingleWorker) {
			const swconst = constraint as RenderedConstraint_SingleWorker;
			members_1 = [];
			for (const member_id of swconst.getAssignments().get()) {
				const ra = $stagingProcessedMembers?.rendered_assignables.get(member_id);
				if (ra) {
					members_1.push(ra);
				}
			}
		} else if (constraint.getConstraintClass() == ConstraintClass.MatchOne) {
			members_1 = [];
			const moconst = constraint as RenderedConstraint_MatchOne;
			const tomatch = $stagingProcessedMembers?.rendered_assignables.get(
				moconst.getAssignableToMatch()
			);
			if (tomatch) {
				members_1 = [tomatch];
			}

			members_2 = [];
			const staging_processed_members = $stagingProcessedMembers;
			if (staging_processed_members) {
				const candidate_rais = moconst.getCandidateRAIndices(staging_processed_members);
				candidate_rais.get().forEach((rai) => {
					const ra = staging_processed_members.rendered_assignables.get(rai);
					if (ra) {
						members_2.push(ra);
					}
				});
			}
		}
		console.debug('Populated members:', members_1);
	};

	//If selected, propose changes
	$: {
		if (selected) {
			const selection = $stagingSelection.selections.get(StagingSelectionMode.secondary_selected);

			const proposed_members: RenderedAssignable[] = [];
			selection?.forEach((ra) => {
				if (ra && ra.getStagingType() == StagingTypes.Assignable) {
					proposed_members.push(ra as RenderedAssignable);
				}
			});

			if ($stagingProcessedMembers) {
				if (constraint.getConstraintClass() == ConstraintClass.SingleWorker) {
					const swconst = constraint as RenderedConstraint_SingleWorker;
					swconst.proposeAssignables(proposed_members, $stagingProcessedMembers);
				} else if (constraint.getConstraintClass() == ConstraintClass.MatchOne) {
					const moconst = constraint as RenderedConstraint_MatchOne;
					moconst.proposeAssignableToMatch(proposed_members[0], $stagingProcessedMembers);
				}
			}
		}
	}

	//Populate members in response to constraint changes AFTER proposing changes (above)
	$: {
		if ($stagingProcessedMembers && constraint.getCurrentData()) {
			populateMembers();
		}
	}

	const onClick = (event: MouseEvent) => {
		selected = !selected;
		if (selected) {
			const multi: boolean = constraint.getConstraintClass() == ConstraintClass.SingleWorker;
			stagingSelection.setSecondaryConfig({
				mode: StagingSelectionMode.secondary_selected,
				multi: multi
			});
			stagingSelection.setSelection(
				StagingSelectionMode.secondary_selected,
				new Set<RenderedAssignable>(members_1)
			);
		} else {
			stagingSelection.revertToPrimaryConfig();
		}
	};

	let candidate_dialog_open = false;
	let bnode: BinaryNode;
	const candidateClick = () => {
		const staging_init = $stagingInit;
		if (staging_init) {
			const moconst = constraint as RenderedConstraint_MatchOne;

			const table_meta = staging_init.update_data.meta;
			const column_meta = staging_init.update_data.colmeta;
			const node_data = MatchOneStagingDetailsCandidatesToBNDataClone(
				moconst.getCurrentData().d as MatchOne_StagingDetails
			);
			bnode = new BinaryNode(table_meta, column_meta, node_data);

			console.debug(bnode);
			candidate_dialog_open = true;
		}
	};

	const discardChanges = () => {
		console.debug('Discard.');
		if ($stagingProcessedMembers) {
			constraint.clearProposedDetails($stagingProcessedMembers);
		}
		populateMembers();
		stagingSelection.setSelection(
			StagingSelectionMode.secondary_selected,
			new Set<RenderedAssignable>(members_1)
		);
	};

	onDestroy(() => {
		stagingSelection.revertToPrimaryConfig();
	});
</script>

<div>
	{#if constraint.getConstraintClass() == ConstraintClass.SingleWorker}
		<ConstraintMemberSelector label="Members" members={members_1} {selected} {onClick} />
	{:else if constraint.getConstraintClass() == ConstraintClass.MatchOne}
		<ConstraintMemberSelector label="To Match" members={members_1} {selected} {onClick} />
		<ConstraintMemberSelector
			label="Candidates"
			members={members_2}
			selected={false}
			onClick={candidateClick}
		/>
	{/if}
	<div class="button">
		<Button variant="raised" color="secondary" on:click={discardChanges}>
			<Label>Undo Changes</Label>
		</Button>
	</div>
</div>
<BinaryNodeDialog
	bind:open={candidate_dialog_open}
	title={'Modify Constraint Candidates'}
	parent_node={bnode}
/>
