import { Button, Dialog, DialogActions, DialogContent, Grid } from '@mui/material';
import React, { FC } from 'react';
import { PickList } from '../../../input/picklists/picklist';
import { SideEffectManager, WrappedHook } from '../../../react/WrappedHook';
import { DataType } from '../../data_types';
import { BNData, BinaryNode, INodeFabProps, createChildInputNode } from '../BinaryNode';
import { ModificationParams } from './common';

export interface AddButtonProps {
	key: string;
	parentprops: INodeFabProps;
	change_handler: () => void;
}

interface Selector {
	selector: BNData;
	options: [];
}

interface InstantiatorRef {
	clone_ref: BNData & Selector;
}

interface AddMemberResult {
	dialog: React.ReactElement;
	add_new_button_push_handler: () => void;
}

export const showBNDataAsSelector = (data: BNData) => {};

const addmember = (props: AddButtonProps) => {
	const retval: AddMemberResult = {} as AddMemberResult;

	const propsnode = props.parentprops.node;
	const column_meta = propsnode.getColumnMeta();
	const instantiors_object = propsnode.node_data.i;
	let instantiator: BNData & Selector = column_meta.instantiators[instantiors_object];

	console.debug('addmember Parsing JSON', props, column_meta, instantiors_object, instantiator);
	const base_instantiator_clone: BNData & Selector = JSON.parse(JSON.stringify(instantiator));

	const instantiator_clone_hook = new WrappedHook<InstantiatorRef>({ clone_ref: null }); //clone the instantiator
	let children = [] as React.ReactNode[];

	const discard = () => {
		instantiator_clone_hook.set({ clone_ref: null });
	};

	retval.add_new_button_push_handler = () => {
		console.debug('Add New Handler called.');
		instantiator_clone_hook.set({ clone_ref: base_instantiator_clone });
	};

	const instantiator_clone = instantiator_clone_hook.get().clone_ref;

	if (instantiator_clone !== null) {
		let save: () => void;

		if (instantiator_clone.selector !== undefined && instantiator_clone.options !== undefined) {
			console.debug('Building selector dialog.', instantiator_clone);
			const subhandler = () => {
				console.debug('Selection changed.');
			};

			switch (instantiator_clone.selector.d) {
				case DataType.Enum:
					const selector_label = column_meta.labels.map[instantiator_clone.selector.l];
					const selector_options = column_meta.translators[instantiator_clone.selector.t];
					const picklist = PickList.create(
						props.parentprops.node.fullkey + '_' + selector_label,
						selector_label,
						false,
						instantiator_clone.selector,
						'v',
						selector_options,
						subhandler,
						false,
						Object.keys(selector_options)
					);

					save = () => {
						console.debug('Selection is ' + instantiator_clone.selector.v);
						const new_clone = instantiator_clone.options[instantiator_clone.selector.v];
						console.debug('Updating instantiator clone to ', new_clone);
						instantiator_clone_hook.set({ clone_ref: new_clone });
					};

					children = [picklist];

					break;
				default:
					console.error('Unhandled instantiator type!');
			}
		} else {
			console.debug('Building final dialog.', instantiator_clone);

			save = () => {
				instantiator_clone_hook.set({ clone_ref: null });
				propsnode.node_data.c.push(instantiator_clone);
				props.change_handler(); //notify parent React component of the change
			};

			const tempnode = new BinaryNode(
				propsnode.table_meta,
				propsnode.column_meta,
				instantiator_clone,
				propsnode.parent_node,
				propsnode.top_node,
				'new'
			);
			const modification_side_effects = new SideEffectManager<ModificationParams>();

			modification_side_effects.addSideEffect('update dialog', () => {
				//const current_parent_node = nodehook.get();
				//const new_parent_node = current_parent_node.buildUpdatedClone();
				//nodehook.set(new_parent_node);
				//console.debug("modification side effect build node",new_parent_node);
				instantiator_clone_hook.set({ clone_ref: instantiator_clone });
			});

			children = [
				createChildInputNode({
					node: tempnode,
					modification_side_effects: modification_side_effects
				})
			];
		}

		retval.dialog = (
			<Dialog
				key="build dialog"
				open={true}
				fullWidth={false}
				disableEscapeKeyDown={true}
				fullScreen={false}
				aria-labelledby="form-dialog-title"
			>
				<DialogContent>{children}</DialogContent>
				<DialogActions>
					<Button
						id={props.parentprops.node.fullkey + '_add_save'}
						onClick={save}
						color="primary"
						variant="contained"
					>
						Save
					</Button>
					<Button
						id={props.parentprops.node.fullkey + '_add_discard'}
						onClick={discard}
						color="primary"
						variant="contained"
					>
						Discard
					</Button>
				</DialogActions>
			</Dialog>
		);
	}
	return retval;
};

export const AddMemberButton: FC<AddButtonProps> = (props: AddButtonProps) => {
	const amr = addmember(props);

	return (
		<Grid key="add button" container item xs={6} direction="row" justifyContent="flex-end">
			<Grid item xs={'auto'}>
				<Button
					id={props.parentprops.node.fullkey + '_addnew'}
					key="button"
					onClick={amr.add_new_button_push_handler}
					color="primary"
					variant="contained"
				>
					Add New
				</Button>
				{amr.dialog}
			</Grid>
		</Grid>
	);
};
