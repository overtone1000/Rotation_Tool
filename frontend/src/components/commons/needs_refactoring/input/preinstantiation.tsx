import { Button, Dialog, DialogActions, DialogContent, Typography } from '@mui/material';
import React from 'react';
import { ASDisplayResponseData } from '../ajax/commands';
import { BinaryNode, BNData, createChildInputNode } from '../data_processing/binary/BinaryNode';
import { Display_Main_Props } from '../displays/display';
import { BuildDialogBaseProps, ShowElementDialog_Base } from './dialogs';

export type SimpleInstantiator = { [i: string]: any };

export interface Predecision {
	l?: string;
	meta_col_key: string;
	bn_data: BNData;
	copy_behavior: CopyBehavior;
}

enum CopyBehavior {
	ForceCopy,
	ForceSelection
}

export interface RamificationTrigger {
	predes_key: string;
	trigger_values: [];
}

export interface Ramification {
	rt: 'set_col' | 'sub_ramification' | 'sub_decisions' | 'set_child'; //ramification_type
	predes_key: string;
	delta_col_key: string;
	options?: [{}];
	child_keys?: number[];
	ramification_triggers?: RamificationTrigger[];
	sub_decision_data?: PredecisionData;
}

export type PredecisionMap = { [i: string]: Predecision };

export interface PredecisionsAndRamifications {
	pre_decisions: PredecisionMap;
	ramifications: Ramification[];
}

export type Instantiation = { [i: string]: any };

export interface PredecisionData extends PredecisionsAndRamifications {
	init: Instantiation;
}

export type Instantiator = SimpleInstantiator | PredecisionData;

export const processInstantiator = (props: Display_Main_Props, init?: Instantiation) => {
	console.debug('Building new item dialog.', props, init);

	const update_data: ASDisplayResponseData = props.response.update_data as ASDisplayResponseData;

	const cloned_instantiator: Instantiator = JSON.parse(
		JSON.stringify(update_data.meta.i)
	) as Instantiator;
	console.debug('Update data:', update_data);
	console.debug('Cloned instantiator:', cloned_instantiator);

	const copy: boolean = init !== undefined;

	if (cloned_instantiator.pre_decisions === undefined) {
		if (copy) {
			for (const col in init) {
				cloned_instantiator[col] = init[col];
			}
			console.debug('Copied', cloned_instantiator, init);
		}

		show_final_dialog(cloned_instantiator as SimpleInstantiator, props);
	} else {
		const pdd: PredecisionData = cloned_instantiator as PredecisionData;

		const pdar: PredecisionsAndRamifications = {
			pre_decisions: pdd.pre_decisions,
			ramifications: pdd.ramifications
		};

		if (copy) {
			for (const col in init) {
				pdd.init[col] = init[col];
			}
			console.debug('Copied', pdd.init, init);
		}

		showPredecisionDialog(pdar, pdd.init, props, copy).then(() => {
			show_final_dialog(pdd.init, props);
		});
	}
};

const show_final_dialog = (
	final_cloned_instantiator: SimpleInstantiator,
	props: Display_Main_Props
) => {
	const update_data: ASDisplayResponseData = props.response.update_data as ASDisplayResponseData;

	console.debug('Showing final dialog for new member.', final_cloned_instantiator, update_data);

	const baseprops = BuildDialogBaseProps(
		'New Element',
		final_cloned_instantiator,
		update_data,
		props.app
	);

	const save_handler = (evt: any) => {
		console.debug('Save new handler called.', final_cloned_instantiator);
		props.app.commitNew(final_cloned_instantiator);
	};

	const final_dialog = ShowElementDialog_Base(baseprops, null, save_handler);

	props.app.showDialog(final_dialog);
};

const showPredecisionDialog = (
	predecisions_and_ramifications: PredecisionsAndRamifications,
	instantiation: Instantiation,
	props: Display_Main_Props,
	copy: boolean,
	previous_predecisions?: PredecisionMap
) => {
	const retval = new Promise<void>((resolve, reject) => {
		console.debug('Processing predecisions:', predecisions_and_ramifications, instantiation, props);
		const children: React.ReactElement[] = [];
		for (const predes_key in predecisions_and_ramifications.pre_decisions) {
			const predes = predecisions_and_ramifications.pre_decisions[predes_key];
			const table_meta = (props.response.update_data as ASDisplayResponseData).meta;
			const column_meta = (props.response.update_data as ASDisplayResponseData).cols[
				predes.meta_col_key
			].meta;
			const binnode = new BinaryNode(
				table_meta,
				column_meta,
				predes.bn_data,
				undefined,
				undefined,
				predes_key
			);

			if (!copy || predes.copy_behavior == CopyBehavior.ForceSelection) {
				if (predes.l != undefined) {
					children.push(<Typography key={'label_' + predes_key}>{predes.l}</Typography>);
				}
				children.push(createChildInputNode({ node: binnode }));
			}
		}

		const save = () => {
			console.debug('Processing ramifications.', predecisions_and_ramifications);
			let last_promise = Promise.resolve();
			for (const ramification of predecisions_and_ramifications.ramifications) {
				last_promise = last_promise.then(() =>
					processRamification(
						ramification,
						predecisions_and_ramifications,
						instantiation,
						props,
						copy,
						previous_predecisions
					)
				);
			}
			last_promise.then(resolve, reject);
		};

		//If the dialog is empty, call save and quit!
		if (children.length <= 0) {
			save();
		}

		const discard = () => {
			props.app.showDialog(null);
			console.debug('Rejecting predecision dialog promise.');
			reject();
		};

		const dialog = (
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
					<Button id={'pred_save_button'} onClick={save} color="primary" variant="contained">
						Save
					</Button>
					<Button id={'pred_discard_button'} onClick={discard} color="primary" variant="contained">
						Discard
					</Button>
				</DialogActions>
			</Dialog>
		);

		console.debug('Showing predecision dialog.', predecisions_and_ramifications);
		props.app.showDialog(dialog);
	});
	return retval;
};

const processRamification = (
	ramification: Ramification,
	predecisions_and_ramifications: PredecisionsAndRamifications,
	instantiation: Instantiation,
	props: Display_Main_Props,
	copy: boolean,
	previous_predecisions?: PredecisionMap
) => {
	const getPredecision = (predecision_key: string) => {
		if (predecision_key == undefined) {
			return undefined;
		}
		console.debug('Getting predecision', predecision_key);
		let predecision = predecisions_and_ramifications.pre_decisions[predecision_key];
		if (!predecision && previous_predecisions) {
			predecision = previous_predecisions[predecision_key];
		}
		return predecision;
	};

	const getPredecisionValue = (predecision: Predecision, predecision_key: string) => {
		if (predecision == undefined || predecision_key == undefined) {
			return undefined;
		}
		console.debug('Getting predecision value', predecision, predecision_key);
		let value = null;
		if (ramification.options) {
			console.debug('Setting value based on ramification options.', predecision, ramification);
			value = ramification.options[predecision.bn_data.v];
		} else {
			console.debug('No options. Setting value based on raw predecision value.', predecision);
			value = predecision.bn_data.v;
		}

		return value;
	};

	const retval = new Promise<void>((resolve, reject) => {
		const primary_predecision = getPredecision(ramification.predes_key);
		const primary_value = getPredecisionValue(primary_predecision, ramification.predes_key);

		//If this is a copy and the copy behavior is to force the copy value, ignore this ramification.
		if (copy && primary_predecision.copy_behavior == CopyBehavior.ForceCopy) {
			switch (ramification.rt) {
				case 'set_col':
				case 'set_child':
					console.debug(
						'Copy mode is force copy, ramification type is ' +
							ramification.rt +
							'. Ignoring Ramification.'
					);
					resolve();
				default:
					break;
			}
		}

		console.debug(
			'Processing ramification:',
			ramification,
			primary_predecision,
			predecisions_and_ramifications,
			props
		);

		//Escape if ramification triggers exist that aren't met
		if (ramification.ramification_triggers !== undefined) {
			console.debug('Checking triggers.', ramification.ramification_triggers);
			for (const trigger of ramification.ramification_triggers) {
				let this_trigger_triggered = false;
				const predecision = getPredecision(trigger.predes_key);
				const value = getPredecisionValue(predecision, trigger.predes_key);
				console.debug('Checking trigger:', predecision, value, trigger);
				for (const trigger_value of trigger.trigger_values) {
					if (trigger_value == value) {
						console.debug('Checking trigger:true', trigger_value, value);
						this_trigger_triggered = true;
						break;
					}
				}
				if (!this_trigger_triggered) {
					console.debug('Checking trigger:false');
					resolve();
				} //If a trigger isn't true, the ramification isn't triggered. Resolve immediately.
			}
		}

		//Else, proceed with ramification
		switch (ramification.rt) {
			case 'set_col':
				console.debug('Setting column ', ramification.delta_col_key, ' to value ', primary_value);
				instantiation[ramification.delta_col_key] = primary_value;
				break;
			case 'sub_ramification':
				const srp = processRamification(
					primary_value as Ramification,
					predecisions_and_ramifications,
					instantiation,
					props,
					copy,
					previous_predecisions
				);
				srp.then(resolve, reject);
				return;
			case 'set_child':
				console.debug(
					'Set child called.',
					instantiation,
					ramification,
					predecisions_and_ramifications,
					ramification.child_keys
				);
				let final_parent = instantiation[ramification.delta_col_key];
				for (let n = 0; n < ramification.child_keys.length - 1; n++) {
					final_parent = final_parent.c[ramification.child_keys[n]];
				}
				final_parent.c[ramification.child_keys[ramification.child_keys.length - 1]] = primary_value;
				break;
			case 'sub_decisions':
				//Need to pass along previous predecisions here.
				console.debug('Showing subdecision dialog');
				const prior_decisions: PredecisionMap = {
					...predecisions_and_ramifications.pre_decisions,
					...previous_predecisions
				};
				const pdp = showPredecisionDialog(
					ramification.sub_decision_data,
					instantiation,
					props,
					copy,
					prior_decisions
				);
				pdp.then(resolve, reject);
				return;
		}

		resolve(); //Default is just to resolve when done.
	});
	return retval;
};
