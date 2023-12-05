import { Grid } from '@mui/material';
import React, { FC } from 'react';
import {
	AssignmentSegmentDisplay,
	AssignmentSegmentDisplayProps
} from '../../../displays/AssignmentSegmentDisplay';
import { WrappedHook } from '../../../react/WrappedHook';
import { BinaryNode, INodeFabProps } from '../BinaryNode';
import { assembleNode } from './common';

export const AssignmentSegmentDisplayInputNode: FC<INodeFabProps> = function (
	props: INodeFabProps
) {
	const segment_select = new WrappedHook<string>(null);

	props.modification_side_effects.addSideEffect('segment select null', (e) => {
		console.debug('Added side effect working!!!', props);
		if (e.resetsegmentdisplay) {
			segment_select.set(null);
		}
	});

	const propsnode = props.node;

	propsnode.handlers = {
		segmentSelectHandler: (segment_key: string | null) => {
			segment_select.set(segment_key);
			console.debug('Side effect function called. Segment selected: ' + segment_key);
		},
		assignmentSelectHandler: undefined,
		constraintSelectHandler: undefined
	};

	const segments_children = propsnode.children;
	const selected_segment = segment_select.get();
	if (selected_segment !== null) {
		for (const segment_key in segments_children) {
			const segment_node: BinaryNode = segments_children[segment_key];
			if (selected_segment == segment_key) {
				segment_node.hidden = false;
			} else {
				segment_node.hidden = true;
			}
		}
	} else {
		for (const segment_key in segments_children) {
			const segment_node: BinaryNode = segments_children[segment_key];
			segment_node.hidden = false;
		}
	}

	let additional_nodes: React.ReactNode[] = new Array<React.ReactNode>(0);

	console.debug('Making assignment segment display.');
	const column_meta = propsnode.column_meta;
	const ss_props: AssignmentSegmentDisplayProps = {
		key: 'segment_display',
		members: propsnode.node_data.c,
		column_meta: column_meta,
		individual_rows: true,
		syncer: undefined,
		segment_select_hook: segment_select //propsnode.handlers.segmentSelectHandler
	};
	const segmentdisplay = React.createElement(AssignmentSegmentDisplay, ss_props, null);
	additional_nodes.push(
		<Grid
			container
			item
			xs={12}
			key="segment_display_grid"
			direction="row"
			alignItems="center"
			justifyContent="center"
			maxWidth="100%"
		>
			{segmentdisplay}
		</Grid>
	);

	return assembleNode({
		props: props,
		child_node_width: 'auto',
		additional_nodes: additional_nodes
	});
};
