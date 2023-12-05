import { Grid } from '@mui/material';
import React, { FC } from 'react';
import {
	AssignmentSelectionState,
	ConstraintSelectionState,
	ScheduleTemplateDisplay,
	ScheduleTemplateDisplayProps
} from '../../../displays/ScheduleTemplateSegmentDisplay';
import { WrappedHook } from '../../../react/WrappedHook';
import { ScheduleTemplateDetails } from '../../extended_types/bndata/ScheduleTemplateDetails';
import { BinaryNode, INodeFabProps } from '../BinaryNode';
import { assembleNode } from './common';

export const ScheduleTemplateDetailsInputNode: FC<INodeFabProps> = function (props: INodeFabProps) {
	const assignment_select = new WrappedHook<AssignmentSelectionState>(null);
	const constraint_select = new WrappedHook<ConstraintSelectionState>(null);

	props.modification_side_effects.addSideEffect('sg select null', (e) => {
		if (e.resetsegmentdisplay) {
			assignment_select.set(null);
			constraint_select.set(null);
		}
	});

	const propsnode = props.node;

	propsnode.handlers = {
		segmentSelectHandler: undefined,
		assignmentSelectHandler: (state: AssignmentSelectionState) => {
			assignment_select.set(state);
			console.debug('Side effect funciton called. Assignment selected: ' + state);
		},
		constraintSelectHandler: (state: ConstraintSelectionState) => {
			constraint_select.set(state);
			console.debug('Side effect funciton called. Constraint selected: ' + state);
		}
	};

	const assignment_members_children = propsnode.children[0].children;
	const constraint_members_children = propsnode.children[1].children;

	const assignmentselect = assignment_select.get();
	if (assignmentselect) {
		//console.debug("Assignment select is " + assignmentselect);
		for (const assignment_key in assignment_members_children) {
			const assignment_node: BinaryNode = assignment_members_children[assignment_key];
			if (assignmentselect == assignment_key) {
				assignment_node.hidden = false;
			} else {
				assignment_node.hidden = true;
			}
		}
	} else {
		for (const assignment_key in assignment_members_children) {
			const assignment_node: BinaryNode = assignment_members_children[assignment_key];
			assignment_node.hidden = false;
		}
	}

	const constraintselect = constraint_select.get();
	if (constraintselect) {
		//console.debug("Constraint select is " + assignmentselect);
		for (const constraint_key in constraint_members_children) {
			const constraint_node: BinaryNode = constraint_members_children[constraint_key];
			if (constraintselect == constraint_key) {
				constraint_node.hidden = false;
			} else {
				constraint_node.hidden = true;
			}
		}
	} else {
		for (const constraint_key in constraint_members_children) {
			const constraint_node: BinaryNode = constraint_members_children[constraint_key];
			constraint_node.hidden = false;
		}
	}

	let additional_nodes: React.ReactNode[] = new Array<React.ReactNode>(0);

	console.debug('Making schedule template display.');
	const column_meta = propsnode.column_meta;
	const sg_props: ScheduleTemplateDisplayProps = {
		key: 'scheduletemplatedisplay',
		data: new ScheduleTemplateDetails(propsnode.node_data),
		column_meta: column_meta,
		table_meta: propsnode.table_meta,
		assignment_select_hook: assignment_select, //propsnode.handlers.assignmentSelectHandler,
		constraint_select_hook: constraint_select //propsnode.handlers.constraintSelectHandler
	};
	const segmentdisplay = ScheduleTemplateDisplay(sg_props); //React.createElement(ScheduleTemplateDisplay,sg_props,null);
	additional_nodes.push(
		<Grid
			key={'sd' + propsnode.local_key}
			container
			item
			xs={12}
			direction="row"
			alignItems="center"
			justifyContent="center"
			maxWidth="100%"
		>
			{segmentdisplay}
		</Grid>
	);

	return assembleNode({ props: props, child_node_width: 6, additional_nodes: additional_nodes });
};
