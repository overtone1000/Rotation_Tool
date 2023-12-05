import ArrowDropDownIcon from '@mui/icons-material/ArrowDropDown';
import ArrowDropUpIcon from '@mui/icons-material/ArrowDropUp';
import DeleteIcon from '@mui/icons-material/Delete';
import { Box, Button, ButtonGroup, Grid, GridSize, IconButton, Typography } from '@mui/material';
import React from 'react';
import { BF_Label } from '../../../../refactored/commons/constants';
import { GenericInputField } from '../../../input/dialogs';
import { PickList } from '../../../input/picklists/picklist';
import { autoscheda_theme } from '../../../theming/theme';
import { DataType } from '../../data_types';
import { Constraint } from '../../extended_types/bndata/Constraint';
import { BinaryNode, BNData, createChildInputNode, INodeFabProps } from '../BinaryNode';
import { AddButtonProps, AddMemberButton } from './addmember';

export interface ModificationParams {
	resetsegmentdisplay?: boolean;
}

const buildDeleteButton = (remove_handler: () => void) => {
	return (
		//<Grid key="delete" container item xs={colwidth} direction="row" justifyContent="flex-end"  alignItems="center" maxWidth="100%">
		//<Grid item xs={"auto"}>
		<IconButton onClick={remove_handler} color="inherit">
			<DeleteIcon />
		</IconButton>
		//</Grid>
		//</Grid>
	);
};

const buildReorderButtons = (change_order_handler: (change: number) => void, node: BinaryNode) => {
	const this_index = parseInt(node.local_key);
	const last_index = Object.keys(node.parent_node.children).length - 1;
	const up_enabled = this_index > 0;
	const down_enabled = this_index < last_index;
	return (
		//<Grid key="change_order" container item xs={colwidth} direction="row" justifyContent="center"  alignItems="center" maxWidth="100%">
		<ButtonGroup size="small" orientation="vertical">
			<Button key="up" disabled={!up_enabled}>
				<ArrowDropUpIcon
					fontSize="small"
					onClick={() => {
						change_order_handler(-1);
					}}
				/>
			</Button>
			<Button key="down" disabled={!down_enabled}>
				<ArrowDropDownIcon
					fontSize="small"
					onClick={() => {
						change_order_handler(1);
					}}
				/>
			</Button>
		</ButtonGroup>
		//</Grid>
	);
};

/*
const buildInstantiableChildButtons = (key:string|number, colwidth:"auto"|3, remove_handler:()=>void, change_order_handler:(change:number)=>void) => {
    
    const retval:React.ReactNode[] = [];

    retval.push(buildReorderButtons(key,colwidth,change_order_handler));
    retval.push(buildDeleteButton(key,colwidth,remove_handler));
    
    return retval;
};
*/

interface InputMembers {
	toprow: React.ReactNode | undefined;
	leftcol: React.ReactNode | undefined;
	rightcol: React.ReactNode | undefined;
	main: React.ReactNode | undefined;
}

export const getInputMembers = function (props: INodeFabProps): InputMembers {
	const propsnode = props.node;
	const column_meta = propsnode.getColumnMeta();
	const retval: InputMembers = {} as InputMembers;
	//React.ReactNode[] = new Array<React.ReactNode>(0);

	let label: string = null;

	if (propsnode.node_data.l !== undefined) {
		label = propsnode.getLabel();
		if (
			propsnode.node_data.l == BF_Label.AssignmentMember ||
			propsnode.node_data.l == BF_Label.ConstraintMember
		) {
			label = label + ' ' + propsnode.local_key;
		}
	}

	let remove_handler = null;
	let change_order_handler = null;
	if (propsnode.parent_node !== null && propsnode.parent_node.node_data.i !== undefined) {
		//This is an instantiable child!
		console.debug('Creating delete handler.');
		remove_handler = () => {
			const index = propsnode.parent_node.node_data.c.indexOf(propsnode.node_data);
			console.debug('Index is ' + index);
			if (index !== null) {
				console.debug('Current children:', propsnode.parent_node.node_data.c);
				const deleted = propsnode.parent_node.node_data.c.splice(index, 1);
				console.debug('Target instance.', propsnode.node_data);
				console.debug('Deleted instance.', deleted);
				console.debug('New children:', propsnode.parent_node.node_data.c);

				//Need special handling for Assignment Member removal as NodeReference datatype will need to change.
				//This needs to happen in the UI, not the backend.
				//If the following is true, this is an assignment member of a schedule template, so adjust indices
				//If propsnode.parent_node.parent_node is null, this is actually a staging match one constraint, so just skip this
				if (
					propsnode.node_data.l == BF_Label.AssignmentMember &&
					propsnode.parent_node.parent_node !== null
				) {
					const details_node = propsnode.parent_node.parent_node;
					for (const child_index in details_node.children) {
						const child = details_node.children[child_index];
						if (child.node_data.l == BF_Label.ConstraintMembers) {
							for (
								let n = child.node_data.c.length - 1;
								n >= 0;
								n-- //Go backwards for safe splicing
							) {
								const constraint_node = child.node_data.c[n];
								const constraint = new Constraint(constraint_node);
								const constraint_requires_deletion =
									constraint.details.handleAssignmentDeletion(index);
								if (constraint_requires_deletion) {
									console.debug('Splicing out constraint ' + n);
									child.node_data.c.splice(n, 1);
								}
							}
							console.debug('New constraint list', child.node_data.c);
						}
					}
				}

				props.modification_side_effects.trigger({ resetsegmentdisplay: true }, propsnode); //notify parent React component of the change
			}
		};

		change_order_handler = (change: number) => {
			const index = propsnode.parent_node.node_data.c.indexOf(propsnode.node_data);
			console.debug('Index is ' + index);
			if (index !== null) {
				const swap_with = index + change;
				if (swap_with >= 0 && swap_with < propsnode.parent_node.node_data.c.length) {
					const this_member = propsnode.parent_node.node_data.c[index];
					const swap_member = propsnode.parent_node.node_data.c[swap_with];
					propsnode.parent_node.node_data.c[index] = swap_member;
					propsnode.parent_node.node_data.c[swap_with] = this_member;

					//Need special handling for Assignment Member removal as NodeReference datatype will need to change.
					//This needs to happen in the UI, not the backend.
					//If the following is true, this is an assignment member of a schedule template, so adjust indices
					//If propsnode.parent_node.parent_node is null, this is actually a staging match one constraint, so just skip this
					if (
						propsnode.node_data.l == BF_Label.AssignmentMember &&
						propsnode.parent_node.parent_node !== null
					) {
						const details_node = propsnode.parent_node.parent_node;
						for (const child_index in details_node.children) {
							const child = details_node.children[child_index];
							if (child.node_data.l == BF_Label.ConstraintMembers) {
								for (const constraint_node of child.node_data.c) {
									const constraint = new Constraint(constraint_node);
									constraint.details.handleAssignmentIndexSwap(index, swap_with);
								}
							}
						}
					}

					console.debug('Triggering side effects.');
					props.modification_side_effects.trigger({ resetsegmentdisplay: true }, propsnode); //notify parent React component of the change
				}
			}
		};
	}

	const change_handler = () => {
		if (props.modification_side_effects) {
			props.modification_side_effects.trigger({});
		}
	};

	if (propsnode.node_data.v !== undefined) {
		//Node has a defined value
		switch (propsnode.node_data.d) {
			case DataType.Binary:
				console.debug('Unhandled binary type with a defined value. Should never happen?');
				break;
			case DataType.Enum:
				{
					const options = propsnode.column_meta.translators[propsnode.node_data.t];
					const torders = propsnode.column_meta.translator_orders;
					let options_order;
					if (torders) {
						options_order = torders[propsnode.node_data.t];
					}
					console.debug('Building enum.', propsnode.column_meta);
					retval.main = (
						<Grid item key="input" xs={'auto'} maxWidth={'100%'}>
							{PickList.create(
								props.node.fullkey + '_' + label,
								label,
								propsnode.node_data.r,
								propsnode.node_data,
								'v',
								options,
								change_handler,
								false,
								options_order
							)}
						</Grid>
					);
				}
				break;
			case DataType.NodeReference:
				{
					let referenced_node: BNData;
					switch (propsnode.node_data.l as BF_Label) {
						case BF_Label.ConstraintAssignmentMember:
						case BF_Label.ConstraintMultiassignmentMember:
							referenced_node = propsnode.top_node.node_data.c[0]; //.c[node.node_data.v];
							break;
					}
					let options: any = {};
					//console.debug("Referenced node:");
					//console.debug(referenced_node);
					for (const child_index in referenced_node.c) {
						const child = referenced_node.c[child_index].c[0];
						if (column_meta.translators[child.t] !== undefined) {
							const translated_child_value = column_meta.translators[child.t][child.v];
							options[child_index] = child_index + ': ' + translated_child_value;
						} else {
							console.error('Undefined translator.', child.t, child.v, column_meta);
							options[child_index] = child_index + ': ' + child.v;
						}
					}
					//console.debug("Options:");
					//console.debug(options);
					retval.main = (
						<Grid item key="input" xs={'auto'} maxWidth={'100%'}>
							{PickList.create(
								props.node.fullkey + '_' + label,
								label,
								propsnode.node_data.r,
								propsnode.node_data,
								'v',
								options,
								change_handler,
								false,
								Object.keys(options)
							)}
						</Grid>
					);
				}
				break;
			case DataType.Array:
			case DataType.DynamicOptionList:
				console.debug('Unhandled array type.');
				break;
			default:
				retval.main = (
					<Grid item key="input" xs={'auto'} maxWidth={'100%'}>
						{GenericInputField(
							props.node.fullkey + '_' + label,
							label,
							propsnode.node_data.d,
							propsnode.node_data.r,
							propsnode.node_data,
							'v',
							change_handler
						)}
					</Grid>
				);
				break;
		}

		if (
			propsnode.parent_node !== null &&
			propsnode.parent_node.node_data.i !== undefined &&
			!propsnode.parent_node.node_data.r
		) {
			//This is an instantiable child and its parent is not read only
			retval.leftcol = buildReorderButtons(change_order_handler, propsnode);
			retval.rightcol = buildDeleteButton(remove_handler);
		}
	} //This node does not have a defined value
	else {
		const toprowmembers = [] as React.ReactNode[];
		toprowmembers.push(
			<Grid
				key="label"
				container
				item
				xs={6}
				direction="row"
				justifyContent="flex-start"
				maxWidth="100%"
				style={{
					paddingLeft: 7,
					paddingRight: 7,
					paddingTop: 4,
					paddingBottom: 4
				}}
			>
				<Grid item xs={'auto'}>
					<Typography variant="larger">{label}</Typography>
				</Grid>
			</Grid>
		);

		if (
			propsnode.node_data.d !== DataType.DynamicOptionList &&
			propsnode.node_data.i !== undefined
		) {
			//This has instantiable children!
			const ambprops: AddButtonProps = {
				key: 'add button',
				parentprops: props,
				change_handler: change_handler
			};
			toprowmembers.push(React.createElement(AddMemberButton, ambprops, null));
		}

		if (
			propsnode.parent_node !== null &&
			propsnode.parent_node.node_data.i !== undefined &&
			!propsnode.parent_node.node_data.r
		) {
			//This is an instantiable child, and it's parent is not read only
			retval.leftcol = buildReorderButtons(change_order_handler, propsnode);
			retval.rightcol = buildDeleteButton(remove_handler);
		}
		retval.toprow = (
			<Grid
				key="toprow"
				container
				item
				xs={12}
				direction="row"
				justifyContent="space-between"
				alignItems="center"
				maxWidth="100%"
			>
				{toprowmembers}
			</Grid>
		);

		/*
        switch(propsnode.node_data.d)
        {
            case DataType.DynamicOptionList:
                console.error("Need to add InputChildren (per function below) for missing members!")
                member_nodes.push(<DynamicOptionList 
                    column_meta={column_meta}
                    node_data={propsnode.node_data}
                />
                );
                break;
        }
        */
	}

	return retval;
};

export const getInputChildren = function (
	props: INodeFabProps,
	children_width: GridSize
): React.ReactNode[] {
	let child_nodes: React.ReactNode[] = new Array<React.ReactNode>(0);
	const propsnode = props.node;

	let ckey: number = 0;
	if (propsnode.node_data.c != undefined) {
		const column_meta = propsnode.getColumnMeta();
		const label = propsnode.getLabel();
		for (const child_node_key in propsnode.children) {
			const child_node: BinaryNode = propsnode.children[child_node_key];
			const child_result: React.ReactNode = createChildInputNode({
				node: child_node,
				modification_side_effects: props.modification_side_effects
			});
			//
			//
			child_nodes.push(
				<Grid
					item
					xs={children_width}
					key={'input_child' + (ckey++).toString()}
					//maxWidth="100%"
					//sx={{maxWidth:"100%"}} //Needed to make sure it doesn't get wider than viewport
					style={{ maxWidth: '100%' }} //Needed to make sure it doesn't get wider than viewport
				>
					{child_result}
				</Grid>
			);
		}
	}

	return child_nodes;
};

export interface NodeAssemblyProps {
	props: INodeFabProps;
	child_node_width: GridSize;
	additional_nodes?: React.ReactNode[];
}

export const assembleNode = function (params: NodeAssemblyProps): React.ReactElement {
	console.debug('Assembling node', params);
	const propsnode = params.props.node;

	if (propsnode.node_data.h) {
		return null;
	}
	const this_node_content = getInputMembers(params.props);
	const child_nodes = getInputChildren(params.props, params.child_node_width);

	const box_style: React.CSSProperties = {
		margin: 2,
		padding: 2,
		maxWidth: '100%',
		background: autoscheda_theme.palette.background.default
	};

	const displayed_nodes = [] as React.ReactNode[];

	//Top row
	if (this_node_content.toprow) {
		displayed_nodes.push(
			<Grid
				key="top_row"
				container
				item
				xs={12}
				direction="row"
				alignItems="center"
				justifyContent="flex-start"
				maxWidth="100%"
			>
				{this_node_content.toprow}
			</Grid>
		);
	}

	//Main content (in middle of main row between right and left columns, all under top row)
	const central_content = [] as React.ReactNode[];
	{
		if (this_node_content.main) {
			central_content.push(
				<Grid
					key="main"
					container
					item
					xs="auto"
					alignItems="center"
					justifyContent="space-evenly"
					maxWidth="100%"
				>
					{this_node_content.main}
				</Grid>
			);
		}
		if (params.additional_nodes !== undefined && params.additional_nodes.length > 0) {
			central_content.push(
				<Grid
					key="added_nodes"
					container
					item
					xs={12}
					direction="row"
					alignItems="flex-start"
					justifyContent="center"
					maxWidth="100%"
				>
					{params.additional_nodes}
				</Grid>
			);
		}
		if (child_nodes.length > 0) {
			central_content.push(
				<Grid
					key="child_nodes"
					container
					item
					xs={12}
					direction="row"
					alignItems="flex-start"
					justifyContent="center"
					maxWidth="100%"
				>
					{child_nodes}
				</Grid>
			);
		}
	}

	//Main row (whole row below top row containing left column, main content, and right column)
	{
		if (this_node_content.leftcol || this_node_content.rightcol || central_content.length > 0) {
			displayed_nodes.push(
				<Grid key="member_nodes" item xs={12} maxWidth="100%">
					<Box display="flex" flexDirection="row" width="100%" maxWidth="100%" alignItems="center">
						{this_node_content.leftcol}
						<Grid
							key="main"
							container
							alignItems="center"
							justifyContent="space-evenly"
							maxWidth="100%"
							direction="row"
						>
							{central_content}
						</Grid>
						{this_node_content.rightcol}
					</Box>
				</Grid>
			);
		}
	}

	if (params.props.node.node_data.c != undefined) {
		box_style.borderStyle = 'outset';
		box_style.borderColor = autoscheda_theme.palette.primary.dark;
		box_style.borderRadius = 10;
		box_style.borderWidth = 2;
	}
	//flexWrap="wrap" //This is necessary to prevent the item from being too wide

	return (
		<Grid
			key={'assembleNode' + propsnode.local_key}
			item
			xs={'auto'}
			maxWidth="100%"
			style={{ maxWidth: '100%' }}
		>
			<Box style={box_style}>
				<Grid
					container
					key="gridcontainer"
					direction="row"
					alignItems="flex-start"
					justifyContent="flex-start"
					maxWidth="100%" //This is necessary to prevent the item from being too wide
				>
					{displayed_nodes}
				</Grid>
			</Box>
		</Grid>
	);
};
