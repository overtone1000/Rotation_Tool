import { Grid, Paper } from '@mui/material';
import React, { FC } from 'react';
import { createRequestDisplay } from '../../../displays/RequestsDisplay';
import { INodeFabProps } from '../BinaryNode';
import { assembleNode } from './common';

export const RequestInputNode: FC<INodeFabProps> = (props: INodeFabProps) => {
	//console.debug("Rendering request input node.",props);

	const request_display = createRequestDisplay(props.node);

	//const se = (newval:ModificationParams) => {
	//    console.debug("Request input node modification",props);
	//};

	//props.modification_side_effects.addSideEffect("request_modification", se);

	const additional_nodes: React.ReactNode[] = new Array<React.ReactNode>(0);
	additional_nodes.push(
		<Grid
			id={'request_input_node_' + props.node.local_key}
			key={'request_display'}
			container
			item
			xs={12}
			direction="row"
			alignItems="center"
			justifyContent="center"
			maxWidth="100%"
		>
			<Paper
				elevation={5}
				style={{ padding: '10px', alignItems: 'center', justifyContent: 'center' }}
			>
				{request_display}
			</Paper>
		</Grid>
	);

	return assembleNode({
		props: props,
		child_node_width: 'auto',
		additional_nodes: additional_nodes
	});
};
