'use strict';
import { FormControl, InputLabel, MenuItem, Select } from '@mui/material';
import React from 'react';
import { formStyle_generic_inline } from '../../commons/styles';
import {
	ObjectKeyPair_Int,
	ValueBackedFormControl,
	ValueBackedFormControl_Props,
	ValueBackedFormControl_State
} from '../form_elements';

interface PickList_props extends ValueBackedFormControl_Props<number> {
	options: { [key: string]: string };
	allownull: boolean;
	key_order: (number | string)[];
	options_disabled?: (number | string)[];
}

interface PickList_state extends ValueBackedFormControl_State {}

export class PickList extends ValueBackedFormControl<number, PickList_props, PickList_state> {
	static create(
		id: string,
		label: string,
		disabled: boolean,
		value_object: {},
		value_key: string,
		options: { [key: number]: string },
		changeHandler: undefined | (() => void),
		allownull: boolean,
		key_order: (number | string)[],
		options_disabled?: (number | string)[]
	): React.ReactNode {
		console.debug('Creating picklist ', label, value_object, value_key, disabled);
		const vop = new ObjectKeyPair_Int(value_object, value_key);
		return (
			<PickList
				id={id}
				key={value_key}
				label={label}
				data_type="number"
				disabled={disabled}
				obkey={vop}
				initial_value={vop.get()}
				options={options}
				changeHandler={changeHandler}
				allownull={allownull}
				key_order={key_order}
				options_disabled={options_disabled}
			/>
		);
	}

	constructor(props: PickList_props) {
		super(props);
		//this.state={selected_key:Object.keys(this.props.options)[0]};
	}

	/*
  selectionChange(e:any)
  {
    const new_index = e.target.selectedIndex;
    const new_key = Object.keys(this.props.options)[new_index];
    this.setState({selected_key:new_key});
  }
*/

	render() {
		let keyvalarr = [];
		let keylist: (string | number)[];
		if (this.props.key_order) {
			keylist = this.props.key_order;
			console.debug('Keylist is', keylist);
		} else {
			keylist = Object.keys(this.props.options);
			console.debug('Building keylist from option list.', this.props.options, keylist);
		}

		if (this.props.allownull) {
			keyvalarr.push(
				<MenuItem id={'null'} key={'null'} value={''}>
					{'None'}
				</MenuItem>
			);
		}
		keylist.forEach((option_key: string | number) => {
			let disabled = false;
			if (this.props.options_disabled) {
				disabled = this.props.options_disabled.includes(option_key);
			}
			keyvalarr.push(
				<MenuItem
					id={'mi_' + option_key.toString()}
					key={option_key}
					value={option_key}
					disabled={disabled}
				>
					{this.props.options[option_key]}
				</MenuItem>
			);
		});

		//let value = this.state.value;
		//if(value===null){value=nullkey;}

		const label_id = 'label';
		return (
			<FormControl style={formStyle_generic_inline}>
				<InputLabel key={label_id}>{this.props.label}</InputLabel>
				<Select
					id={this.props.id}
					labelId={label_id}
					label={this.props.label}
					key={'select'}
					className="form-control"
					type="text"
					disabled={this.props.disabled}
					value={this.props.obkey.get()}
					onChange={this.handleValChange}
				>
					{keyvalarr}
				</Select>
			</FormControl>
		);
	}
}
