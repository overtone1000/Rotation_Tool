'use strict';
import {
	Checkbox,
	FormControl,
	Grid,
	IconButton,
	InputLabel,
	ListItemText,
	MenuItem,
	Select
} from '@mui/material';
import { ClearAll, SelectAll } from '@mui/icons-material';
import React, { FC } from 'react';
import { WrappedHook } from '../../react/WrappedHook';
import { getTargetValues, getValues } from './arraytypechanges';
import { Labels } from '../../data_processing/data_types';

export const createBasicMultiplePickList = (
	labels: Labels,
	disabled: boolean,
	changeHandler: undefined | ((newvals: number[] | string[]) => void)
): React.ReactNode => {
	console.debug('Create basic multiple picklist', labels);
	return (
		<FilterSelectPickList
			disabled={disabled}
			options={labels.map}
			order={labels.order}
			changeHandler={changeHandler}
		></FilterSelectPickList>
	);
};

export const ITEM_HEIGHT = 30;
export const ITEM_PADDING_TOP = 8;
export const MenuProps = {
	PaperProps: {
		style: {
			maxHeight: ITEM_HEIGHT * 10 + ITEM_PADDING_TOP,
			width: 250
		}
	}
};

export interface MultiPickListState {
	selected_values: WrappedHook<number[]>;
	keyvalobjects: React.ReactNode[];
}

export const constructMultiPickListState = (
	options: { [key: number]: string },
	order: (number | string)[],
	init: number[]
) => {
	//console.debug("Constructing list, initial selection is",init);
	//console.debug("Options are",options);

	const retval: MultiPickListState = {} as MultiPickListState;
	retval.selected_values = new WrappedHook<number[]>(init);
	retval.keyvalobjects = [];
	console.debug('Selections are', retval.selected_values.get(), 'of', order, init, options);
	for (const option_key of order) {
		const option_val = options[option_key];
		const key_as_int = parseInt(option_key as any);
		retval.keyvalobjects.push(
			<MenuItem id={'cb_' + option_key} key={option_key} value={option_key.toString()}>
				<Checkbox checked={retval.selected_values.get().indexOf(key_as_int) >= 0} />
				<ListItemText primary={option_val} />
			</MenuItem>
		);
	}

	return retval;
};

export interface FilterSelectPickList_Props {
	label?: string;
	options: { [key: number]: string };
	order: (number | string)[];
	disabled: boolean;
	changeHandler: (newvals: number[]) => void;
}

const FilterSelectPickList: FC<FilterSelectPickList_Props> = (
	props: FilterSelectPickList_Props
) => {
	const all_selection: number[] = [];
	for (const key in props.options) {
		all_selection.push(parseInt(key));
	}

	const state: MultiPickListState = constructMultiPickListState(
		props.options,
		props.order,
		all_selection
	);

	const handleValChange = (newvals: number[]) => {
		state.selected_values.set(newvals);
		props.changeHandler(newvals);
	};

	const renderValue = (value: string[]) => {
		//console.debug("Rendering value ",value);
		if (value.length == Object.keys(props.options).length) {
			return 'All Shown';
		} else if (value.length > 0) {
			return value.length + ' Items Shown';
		} else {
			return 'All Hidden';
		}
	};
	console.debug('FilterSelectPickList', props);

	const label_id = 'label';
	return (
		<Grid
			container
			direction="row"
			justifyContent="flex-start"
			alignItems="center"
			style={{ width: '100%', height: '100%' }}
		>
			<Grid item xs={'auto'}>
				<InputLabel id={label_id} key={label_id}>
					{props.label}
				</InputLabel>
				<Select
					multiple
					labelId={label_id}
					label={props.label}
					className="form-control"
					type="text"
					disabled={props.disabled}
					value={getValues(state.selected_values.get())}
					renderValue={renderValue}
					displayEmpty={true}
					MenuProps={MenuProps}
					onChange={(evt) => {
						console.debug('Evt is', evt.target);
						handleValChange(getTargetValues(evt.target.value as string[]));
					}}
					style={{ height: '100%' }}
				>
					{state.keyvalobjects}
				</Select>
			</Grid>
			<Grid item xs={'auto'}>
				<IconButton aria-label="Select All" onClick={(evt) => handleValChange(all_selection)}>
					<SelectAll />
				</IconButton>
			</Grid>
			<Grid item xs={'auto'}>
				<IconButton aria-label="Select None" onClick={(evt) => handleValChange([])}>
					<ClearAll />
				</IconButton>
			</Grid>
		</Grid>
	);
};
