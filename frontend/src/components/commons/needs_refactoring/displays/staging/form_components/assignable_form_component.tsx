import {
	Autocomplete,
	AutocompleteChangeDetails,
	AutocompleteChangeReason,
	Box,
	TextField
} from '@mui/material';
import React, { FC } from 'react';
import { ASRequestStagingParameters, Staging_Directives } from '../../../ajax/commands';
import { AutoschedaState } from '../../../autoscheda_core';
import { MetaInterpret } from '../../../data_processing/extended_types/UpdateMeta';
import { ASStagingResponseMessage } from '../../../data_processing/staging/stagingdata';
import { RenderedAssignable } from '../members/rendered_assignable';
import { ModificationResponse, panel_style } from '../staging_rightpanel';

export interface AssignableFormComponentProps {
	app: AutoschedaState;
	selection: Set<number>;
	update_data: ASStagingResponseMessage;
	modificationResponseHandler: (e: ModificationResponse) => void;
	rendered_assignable: RenderedAssignable;
}

export const AssignableFormComponent: FC<AssignableFormComponentProps> = (
	props: AssignableFormComponentProps
) => {
	const these_candidates = new Set<number>(props.rendered_assignable.getAssignable().c);
	const this_selected_worker = props.rendered_assignable.getAssignable().w;

	let common_selected_worker_index: number = null;
	let common_candidates: Set<number> = null;

	if (common_candidates === null) {
		common_candidates = these_candidates;
		common_selected_worker_index = this_selected_worker;
	}

	const selection_options = MetaInterpret.getAllWorkers(props.update_data.meta);
	const selection_option_keys = Object.keys(selection_options);
	const selection_enabled = common_candidates !== null && common_candidates.size > 0;

	const selectionChange = (
		event: React.ChangeEvent<{}>,
		value: number,
		reason: AutocompleteChangeReason,
		details?: AutocompleteChangeDetails<any>
	) => {
		if (value === null) {
			return;
		}

		const parameters = {} as ASRequestStagingParameters;
		parameters.type = Staging_Directives.assign;
		parameters.worker_id = value;
		parameters.staging_ids = Array.from(props.selection.values());

		props.app.singleModify(parameters, props.modificationResponseHandler);
	};

	const getOptionLabel = (option) => {
		let worker_name = selection_options[option];
		if (worker_name === undefined) {
			worker_name = null;
		}
		return worker_name;
	};

	const getOptionDisabled = (option) => {
		const option_int = parseInt(option);
		console.debug(
			'Checking option disabled',
			option_int,
			common_candidates,
			!common_candidates.has(option)
		);
		return !common_candidates.has(option_int);
	};

	//console.debug("Autocomplete",selection_option_keys,common_selected_worker_index);

	return (
		<Box>
			<Autocomplete
				key="autocomplete"
				id="worker_select"
				disabled={!selection_enabled}
				value={common_selected_worker_index}
				onChange={selectionChange}
				options={selection_option_keys}
				getOptionLabel={getOptionLabel}
				getOptionDisabled={getOptionDisabled}
				isOptionEqualToValue={(option: string, value: string) => {
					return parseInt(option) === parseInt(value);
				}}
				style={{
					marginTop: '20px',
					marginBottom: '20px',
					width: panel_style.width
				}}
				renderInput={(params) => <TextField {...params} label="Select Worker" variant="outlined" />}
			/>
		</Box>
	);
};
