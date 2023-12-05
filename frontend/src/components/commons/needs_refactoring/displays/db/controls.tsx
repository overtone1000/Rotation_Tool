import { SettingsApplicationsTwoTone } from '@mui/icons-material';
import HighlightOffIcon from '@mui/icons-material/HighlightOff';
import { Box, Grid, IconButton } from '@mui/material';
import FormControl from '@mui/material/FormControl';
import FormGroup from '@mui/material/FormGroup';
import FormLabel from '@mui/material/FormLabel';
import { formatISO, parseISO } from 'date-fns';
import React, { FC } from 'react';
import {
	ASDisplayResponseData,
	ASRequest,
	OperationContents,
	UpdateTypes
} from '../../ajax/commands_generic';
import {
	AutoschedaDisplayParameter,
	AutoschedaState,
	ControlsDisplayModification
} from '../../autoscheda_core';
import { Labels } from '../../data_processing/data_types';
import { createSimpleBooleanControl } from '../../input/form_elements';
import { LinkedDatePicker } from '../../input/LinkedDatePicker';
import { createBasicMultiplePickList } from '../../input/picklists/multipicklist';

export const date_picker_style: React.CSSProperties = {
	marginTop: 3,
	marginBottom: 2
};

const createDatePicker = function (
	id: string,
	label: string,
	key: string,
	initial_date: Date,
	handler: (date: Date) => void
): React.ReactNode {
	console.debug('Creating date picker', label, initial_date, handler);
	return (
		<Box key={key} sx={date_picker_style}>
			<LinkedDatePicker
				id={id}
				disabled={false}
				label={label}
				date={initial_date}
				handler={handler}
			/>
		</Box>
	);
};

/*

*/

export const panel_styles: React.CSSProperties = {
	//marginLeft:3,
	//marginRight:3,
	paddingLeft: 3,
	paddingRight: 3,
	borderLeft: 2,
	borderLeftStyle: 'solid',
	borderLeftColor: 'white'
};

export interface Display_Controls_Props {
	request: ASRequest;
	response: OperationContents;
	app: AutoschedaState;
	display_mod: ControlsDisplayModification;
	display_parameters: AutoschedaDisplayParameter[];
}

export const Controls: FC<Display_Controls_Props> = (props: Display_Controls_Props) => {
	console.debug('Rendering controls.');

	const controlsToggle = () => {
		console.debug('Toggling controls.');
		let newmods = {} as ControlsDisplayModification;
		Object.assign(newmods, props.display_mod);
		newmods.controls_minimized = !newmods.controls_minimized;
		props.app.controls_display_modification.set(newmods);
	};

	const minimized = props.display_mod.controls_minimized;

	if (Object.keys(props.display_parameters).length <= 0) {
		return null;
	}

	let retval: React.ReactElement;
	if (!minimized) {
		let request_elements: React.ReactNode[] = [];
		request_elements = request_elements.concat(addParamChangeHandlers(props));
		request_elements.push(addFilters(props));
		retval = (
			<Box key="Control Panel" flexShrink={0} height="100%" style={panel_styles} display={'block'}>
				<Grid key="top" container justifyContent="flex-end">
					<IconButton name="cp_toggle" onClick={controlsToggle}>
						<HighlightOffIcon fontSize="large" />
					</IconButton>
				</Grid>
				{request_elements}
			</Box>
		);
	} else {
		retval = (
			<Box
				key="Control Panel Sliver"
				flexShrink={0}
				height="100%"
				alignItems="center"
				style={panel_styles}
				display={'flex'}
			>
				<IconButton name="cp_toggle" onClick={controlsToggle} color="secondary">
					<SettingsApplicationsTwoTone fontSize="large" />
				</IconButton>
			</Box>
		);
	}

	console.debug('Finished control prerender.');
	return retval;
};

const addParamChangeHandlers = (props: Display_Controls_Props) => {
	console.debug('addParamChangeHandlers', props);
	let retval: React.ReactNode[] = [];

	for (const paramkey of props.display_parameters) {
		const get_current_param_as_date = () => {
			return parseISO(props.request.parameters[paramkey] as string);
		};
		const datepicker_param_change_handler = (date: Date) => {
			let value = formatISO(date, { representation: 'date' });
			generic_param_change(value);
		};
		const generic_param_change = (value: any) => {
			console.debug('Generic parameter modification', paramkey, value);
			props.app.modifyRequestParameter(paramkey, value);
		};
		const generic_param_change_handler = (evt: any) => {
			console.debug('Generic parameter modification handler', paramkey, evt);
			generic_param_change(evt.target.value);
		};
		const boolean_param_change_handler = (evt: any) => {
			console.debug('Boolean parameter modification handler', paramkey, evt);
			generic_param_change(evt.target.checked);
		};

		switch (paramkey) {
			case 'date':
				{
					const date = get_current_param_as_date();
					retval.push(
						createDatePicker('date', 'Date', paramkey, date, datepicker_param_change_handler)
					);
				}
				break;
			case 'start_date':
				{
					const date = get_current_param_as_date();
					retval.push(
						createDatePicker(
							'start_date',
							'Start Date',
							paramkey,
							date,
							datepicker_param_change_handler
						)
					);
				}
				break;
			case 'end_date':
				{
					const date = get_current_param_as_date();
					retval.push(
						createDatePicker(
							'end_date',
							'End Date',
							paramkey,
							date,
							datepicker_param_change_handler
						)
					);
				}
				break;
			case 'show_retired':
				{
					const showretired = props.request.parameters[paramkey] as boolean;
					retval.push(
						createSimpleBooleanControl(
							'show_retired',
							'Show Retired Members',
							paramkey,
							false,
							showretired,
							boolean_param_change_handler
						)
					);
				}
				break;
			default:
				console.error('Unhandled request element = ' + paramkey);
				break;
		}
	}

	return retval;
};

export const partition_style: React.CSSProperties = {
	width: '100%',
	borderTop: 1,
	borderTopStyle: 'solid',
	borderTopColor: 'white',
	marginTop: 5,
	marginBottom: 3,
	padding: 2
};

const addFilters = (props: Display_Controls_Props) => {
	console.debug('Rendering filters', props);

	const update_data = props.response.update_data as ASDisplayResponseData;

	const changeHandler = (newvals: string[]) => {
		//let current_filters:{[colkey:string]:string[]} = {};
		//Object.assign(current_filters,filters.get());
		//current_filters[colkey]=newvals;
		//filters.set(current_filters);
		console.debug('New filtered columns are', newvals);

		props.app.main_display_modification.set({
			column_filters: newvals
		});
	};

	const createControl = (options: { [key: number]: string }) => {
		console.debug('Creating control', options);
		const order = Object.keys(options).map((value, index, array) => {
			return parseInt(value);
		});
		const labels: Labels = {
			map: options,
			order: order
		};
		return (
			<FormControl key="filters" style={partition_style}>
				<FormLabel key="label">Column Filter</FormLabel>
				<FormGroup key="members">
					{createBasicMultiplePickList(labels, false, changeHandler)}
				</FormGroup>
			</FormControl>
		);
	};

	switch (props.response.update_type) {
		case UpdateTypes.keyed:
			{
				//Remove the first header since that's for the leftmost column, which shouldn't be filterable
				if (update_data.cols) {
					let keys = Object.keys(update_data.cols);
					if (keys.length > 0) {
						const options = {};
						for (const key of keys) {
							options[key] = update_data.cols[key].name;
						}
						return createControl(options);
					}
				}
			}
			break;
		case UpdateTypes.simple:
			{
				//Remove the first header since that's for the leftmost column, which shouldn't be filterable
				const options = {};
				let keys = Object.keys(update_data.headers);
				keys.splice(0, 1);
				for (const key of keys) {
					options[key] = update_data.headers[key];
				}
				return createControl(options);
			}
			break;
		case UpdateTypes.raw:
		default:
			//This doesn't need to be a control. This goes into the column header, not the control panel. See raw.tsx
			return null;
	}
};
