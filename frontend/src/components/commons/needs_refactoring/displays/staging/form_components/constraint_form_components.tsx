import { Box, Chip, ToggleButton, ToggleButtonGroup } from '@mui/material';
import React, { FC, useRef } from 'react';
import { AutoschedaState, WorkManager } from '../../../autoscheda_core';
import { BinaryNode } from '../../../data_processing/binary/BinaryNode';
import {
	ASStagingResponseMessage,
	MatchOneStagingDetailsCandidatesToBNDataClone,
	MatchOne_StagingDetails,
	StagingTypes
} from '../../../data_processing/staging/stagingdata';
import { DialogBaseProps, ShowElementDialog_Base } from '../../../input/dialogs';
import { WrappedHook } from '../../../react/WrappedHook';
import { shortDateString } from '../data_processing/processing01';
import { ProcessingResult02 } from '../data_processing/processing02';
import { RenderedAssignable } from '../members/rendered_assignable';
import {
	GenericRenderedConstraint,
	MatchOneMode,
	RenderedConstraint_MatchOne,
	RenderedConstraint_SingleWorker,
	SelectionChange
} from '../members/rendered_constraint';
import {
	ChangeFocusedSelectionInteraction,
	Interaction,
	InteractionHandler,
	SelectInteraction,
	SelectionHandler
} from '../staging';
import { Revert } from './revert_button';

export interface ConstraintFormComponentProps {
	app: AutoschedaState;
	staging: ProcessingResult02;
	interaction_handler: InteractionHandler;
	rendered_constraint: GenericRenderedConstraint;
	show_revert: boolean;
}

export interface SingleWorkerFormComponentProps extends ConstraintFormComponentProps {
	rendered_constraint: RenderedConstraint_SingleWorker;
}

interface ChippedBoxesProps {
	label: string;
	assignable_set?: Set<number>;
	assigment_members?: { label: string; offset: number }[];
	rendered_assignables: { [i: number]: RenderedAssignable };
}

const ChippedBoxes: FC<ChippedBoxesProps> = (props: ChippedBoxesProps) => {
	const boxes = [];
	let index = 0;
	boxes.push(<Box key="label">{props.label}</Box>);

	if (props.assignable_set != undefined) {
		const sorting_array: RenderedAssignable[] = [];
		for (const index of Array.from(props.assignable_set)) {
			const ra = props.rendered_assignables[index];
			if (ra !== undefined) {
				sorting_array.push(ra);
			} else {
				//This is fine and doesn't need to be handled.
				//console.error("Undefined rendered assignable",index,assignable_set,rendered_assignables);
			}
		}

		sorting_array.sort(RenderedAssignable.compare);

		for (const ra of sorting_array) {
			//ra.getDate().toDateString

			//Add one to month, which is 0 index. Don't add one to day, which is 1 index.
			const datestring = shortDateString(ra.getDate());
			const chip_label = datestring + ' | ' + ra.getAssignableType().getName();
			boxes.push(
				<Box key={index++} style={{ maxWidth: '100%' }}>
					<Chip label={chip_label} style={{ maxWidth: '100%' }} />
				</Box>
			);
		}
	}

	if (props.assigment_members != undefined) {
		console.debug('Rendering assignment members in chipped boxes.', props.assigment_members);
		for (const am of props.assigment_members) {
			boxes.push(
				<Box key={index++} style={{ maxWidth: '100%' }}>
					<Chip label={am.label + ' | ' + am.offset.toString()} style={{ maxWidth: '100%' }} />
				</Box>
			);
		}
	}

	return (
		<Box key="chips" sx={{ width: '100%' }}>
			{boxes}
		</Box>
	);
};

const genericSelectionHandler = (
	props: ConstraintFormComponentProps,
	selection_details: SelectInteraction,
	manager: WorkManager
) => {
	if (selection_details.selection[0].getStagingType() != StagingTypes.Assignable) {
		return;
	}
	const selection_changes = props.rendered_constraint.memberSelectionChange(
		selection_details.selection[0].getIndex()
	);

	handleConstraintMemberChanges(props, selection_changes, manager);
};

const handleConstraintMemberChanges = (
	props: ConstraintFormComponentProps,
	selection_changes: SelectionChange,
	manager: WorkManager
) => {
	for (const this_deselection of selection_changes.getDeselected()) {
		props.staging.rendered_assignables[this_deselection].removeProposedConstraint(
			props.rendered_constraint
		);
	}
	for (const this_selection of selection_changes.getSelected()) {
		props.staging.rendered_assignables[this_selection].addProposedConstraint(
			props.rendered_constraint
		);
	}

	if (props.rendered_constraint.hasProposedDetails()) {
		console.debug('Test02 Starting user work.');
		props.app.getWorkTracker().userWorkStart(manager);
	} else {
		console.debug('Test02 Stopping user work.');
		props.app.getWorkTracker().userWorkStop(manager);
	}

	props.interaction_handler(
		Interaction.changeSecondarySelection,
		props.rendered_constraint.getSecondaryHighlighting()
	);
};

export const SingleWorkerFormComponent: FC<SingleWorkerFormComponentProps> = (
	props: SingleWorkerFormComponentProps
) => {
	console.debug('Rendering single worker form component.');
	let active_element = new WrappedHook<string>(null);
	const work_manager = useRef<WorkManager>(new WorkManager());

	const constraintSelectionHandler: SelectionHandler = (selection_details: SelectInteraction) => {
		return genericSelectionHandler(props, selection_details, work_manager.current);
	};

	const onChange = (event: React.MouseEvent<HTMLElement, MouseEvent>, value: any) => {
		active_element.set(value);

		if (value !== null && value !== undefined) {
			const details: ChangeFocusedSelectionInteraction = {
				handler: { invert: constraintSelectionHandler },
				initial_selections: props.rendered_constraint.getSecondaryHighlighting()
			};
			props.interaction_handler(Interaction.changeFocusedSelection, details);
		} else {
			props.interaction_handler(Interaction.releaseFocusedSelection);
		}
	};

	const assignables = props.rendered_constraint.getEntailedAssignables();

	let savediscard = null;

	if (props.show_revert) {
		savediscard = <Revert parent_props={props} parent_active_element={active_element} />;
	}

	let chipped_boxes = (
		<ChippedBoxes
			label="Assignables"
			assignable_set={assignables}
			rendered_assignables={props.staging.rendered_assignables}
		/>
	);

	let tbg = (
		<ToggleButtonGroup
			style={{ width: '100%' }}
			fullWidth={true}
			key="tbg1"
			size="large"
			color="primary"
			orientation="vertical"
			value={active_element.get()}
			exclusive
			onChange={onChange}
		>
			<ToggleButton style={{ width: '100%' }} key="tb" value="assignables">
				{chipped_boxes}
			</ToggleButton>
		</ToggleButtonGroup>
	);

	return (
		<Box key="swfc" width="100%">
			{tbg}
			{savediscard}
		</Box>
	);
};

export interface MatchOneFormComponentProps extends ConstraintFormComponentProps {
	rendered_constraint: RenderedConstraint_MatchOne;
}

export const MatchOneFormComponent: FC<MatchOneFormComponentProps> = (
	props: MatchOneFormComponentProps
) => {
	console.debug('Rendering match one form component.', props.rendered_constraint);
	let active_element = new WrappedHook<MatchOneMode>(null);
	const work_manager = useRef<WorkManager>(new WorkManager());

	//const assignable_to_match = new SingleSelect(props.rendered_constraint.getAssignableToMatch());
	//const candidates = new MultiSelect(props.rendered_constraint.getCandidates());

	const assignable_to_match_index = props.rendered_constraint.getAssignableToMatch();
	const assignable_to_match_as_set = new Set<number>([assignable_to_match_index]);

	const candidates: { label: string; offset: number }[] = [];

	for (const am of props.rendered_constraint.getCandidateAssignmentMembers()) {
		candidates.push({
			label: props.staging.assignment_types.getType(am.getAssignmentTypeID()).getName(),
			offset: am.getDayOffset()
		});
	}

	const constraintSelectionHandler: SelectionHandler = (selection_details: SelectInteraction) => {
		return genericSelectionHandler(props, selection_details, work_manager.current);
	};

	const onChange = (event: React.MouseEvent<HTMLElement, MouseEvent>, value: any) => {
		active_element.set(value);
		props.rendered_constraint.changeMode(value);
		const moc = props.rendered_constraint as RenderedConstraint_MatchOne;
		const details: ChangeFocusedSelectionInteraction = {
			handler: { invert: constraintSelectionHandler },
			initial_selections: props.rendered_constraint.getSecondaryHighlighting()
		};
		if (value !== null && value !== undefined) {
			switch (value) {
				case MatchOneMode.AssignableToMatch:
					props.interaction_handler(Interaction.changeFocusedSelection, details);
					break;
				case MatchOneMode.CandidateAssignables:
					const constraint_details = props.rendered_constraint.getCurrentData()
						.d as MatchOne_StagingDetails;
					const update_data = props.app.current_response.get()
						.update_data as ASStagingResponseMessage;
					const bndata = MatchOneStagingDetailsCandidatesToBNDataClone(constraint_details);
					console.debug('Cloned MatchOne details', bndata);
					const bn: BinaryNode = new BinaryNode(update_data.meta, update_data.colmeta, bndata);
					console.debug('Created binary node clone', bn);
					const old_candidates = moc.getCandidateRAIndices().get();
					const savehandler = (evt: any) => {
						console.debug('Match One save', bn, bndata, constraint_details);
						moc.changeCandidates(bndata.c);
						const new_candidates = moc.getCandidateRAIndices().get();
						const changes = new SelectionChange();
						for (const oldc of old_candidates) {
							if (!new_candidates.has(oldc)) {
								changes.deselect(oldc);
							}
						}
						for (const newc of new_candidates) {
							if (!old_candidates.has(newc)) {
								changes.select(newc);
							}
						}
						console.debug('Changes:', changes, old_candidates, new_candidates);
						handleConstraintMemberChanges(props, changes, work_manager.current); //Select all candidates
						//props.interaction_handler(Interaction.matchOneCandidateChange, details); //If context is add, will change the proposed addition
					};
					const dialogprops: DialogBaseProps = {
						app: props.app,
						title: 'Edit Match One Candidates',
						content: [bn.createRootInputNode()]
					};
					const dialog = ShowElementDialog_Base(dialogprops, null, savehandler);
					props.app.showDialog(dialog);
					active_element.set(null);
					break;
			}
		} else {
			props.interaction_handler(Interaction.releaseFocusedSelection);
		}
	};

	let savediscard = null;

	if (props.show_revert) {
		savediscard = <Revert parent_props={props} parent_active_element={active_element} />;
	}

	return (
		<Box key="mofc" width="100%">
			<ToggleButtonGroup
				sx={{ width: '100%' }}
				key="tbg1"
				size="large"
				color="primary"
				orientation="vertical"
				value={active_element.get()}
				exclusive
				onChange={onChange}
			>
				<ToggleButton key="tb1" sx={{ width: '100%' }} value={MatchOneMode.AssignableToMatch}>
					<ChippedBoxes
						label="Assignable To Match"
						assignable_set={assignable_to_match_as_set}
						rendered_assignables={props.staging.rendered_assignables}
					/>
				</ToggleButton>
				<ToggleButton key="tb2" sx={{ width: '100%' }} value={MatchOneMode.CandidateAssignables}>
					<ChippedBoxes
						label="Candidates"
						assigment_members={candidates}
						rendered_assignables={props.staging.rendered_assignables}
					/>
				</ToggleButton>
			</ToggleButtonGroup>

			{savediscard}
		</Box>
	);
};
