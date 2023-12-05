import { table } from 'console';
import { id } from 'date-fns/locale';
import { type } from 'os';
import {
	Command_Actions,
	Command_Contexts,
	Command_Parameters,
	Staging_Directives,
	sendPOST,
	type ASRequest,
	type ASRequestStagingParameters,
	type OperationContents,
	type StagingOperationContents
} from '../../../refactored/ajax/commands_generic';
import { localDateToEpochDay } from '../../../refactored/staging/data_processing/processing01';
import type { AddType } from '../../../refactored/staging/data_processing/processing03';
import { Interaction, type ProposedAddition } from '../../../refactored/staging/staging';
import { ConstraintClass } from '../../data_processing/extended_types/bndata/Constraint';
import type {
	ASStagingResponseData,
	ASStagingResponseMessage,
	ConstraintStagingData
} from '../../data_processing/staging/stagingdata';
import { ConfirmDialog, type ConfirmDialogProps } from '../../input/dialogs';

export interface AddBoxProps extends PanelProps {
	//update_data:ASStagingResponseData,
	proposed_addition: ProposedAddition;
	selected_date: Date;
}

export interface EditBoxProps extends PanelProps {
	update_data: ASStagingResponseMessage;
	selection: Set<number>;
}

export interface CommitBoxProps extends PanelProps {
	update_data: ASStagingResponseMessage;
}

export interface Message {
	title: string;
	body: string[];
}

export interface ModificationResponse {
	deletions: {
		assignables: [number];
		constraints: [number];
	};
	updates: ASStagingResponseData;
	messages: Message[];
}

export type StagingModificationResponseHandler = (e: ModificationResponse) => void;
type StagingModificationResponseHandlerBuilderType = (
	props: PanelProps | AddBoxProps
) => StagingModificationResponseHandler;
const stagingModificationResponseHandlerBuilder: StagingModificationResponseHandlerBuilderType = (
	props: PanelProps
) => {
	return (e: ModificationResponse) => {
		console.debug('Response', e);

		props.interaction_handler(Interaction.releaseFocusedSelection);
		props.interaction_handler(Interaction.clearPrimarySelection);

		const abp = props as AddBoxProps;
		if (abp.proposed_addition !== undefined && abp.proposed_addition.constraint !== undefined) {
			abp.proposed_addition.constraint.clearProposedDetails();
			abp.interaction_handler(Interaction.changeProposedAddition, abp.proposed_addition);
		}

		const current_response = props.app.current_response.get() as StagingOperationContents;
		const modified_response = {} as StagingOperationContents;
		Object.assign(modified_response, current_response);

		for (const deleted_assignable of e.deletions.assignables) {
			for (const epoch_day in modified_response.update_data.data.assignables) {
				const asds = modified_response.update_data.data.assignables[epoch_day];
				if (asds !== null && asds !== undefined) {
					const asd = asds[deleted_assignable];
					if (asd !== null && asd !== undefined) {
						delete asds[deleted_assignable];
						break;
					}
				}
			}
		}

		for (const deleted_constraint of e.deletions.constraints) {
			console.debug(
				'Deleting constraint ',
				deleted_constraint,
				modified_response.update_data.data.constraints
			);
			const csd = modified_response.update_data.data.constraints[deleted_constraint];
			if (csd !== null && csd !== undefined) {
				delete modified_response.update_data.data.constraints[deleted_constraint];
				console.debug('Deleted constraint ', modified_response.update_data.data.constraints);
			}
		}

		if (e.updates.assignables) {
			for (const epoch_day in e.updates.assignables) {
				for (const id in e.updates.assignables[epoch_day]) {
					if (!modified_response.update_data.data.assignables[epoch_day]) {
						modified_response.update_data.data.assignables[epoch_day] = {};
					}
					modified_response.update_data.data.assignables[epoch_day][id] =
						e.updates.assignables[epoch_day][id];
				}
			}
		}
		if (e.updates.constraints) {
			for (const id in e.updates.constraints) {
				modified_response.update_data.data.constraints[id] = e.updates.constraints[id];
			}
		}
		if (e.updates.summaries) {
			for (const id in e.updates.summaries) {
				modified_response.update_data.data.summaries[id] = e.updates.summaries[id];
			}
		}

		props.app.current_response.set(modified_response);
		props.app.awaiting_server_response.set(false);
		props.app.getWorkTracker().clear();
	};
};

export interface StagingAdditionData {
	context: AddType;
	selection: number;
	epoch_day: number;
	proposed_constraint?: ConstraintStagingData;
	multiple: number;
}

const addToStaging = (props: AddBoxProps) => {
	console.debug('AddToStaging ', props.proposed_addition);

	const data: StagingAdditionData = {
		context: props.proposed_addition.context,
		selection: props.proposed_addition.selected_type,
		epoch_day: localDateToEpochDay(props.selected_date),
		multiple: props.proposed_addition.multiple
	};

	if (props.proposed_addition.constraint !== undefined) {
		data.proposed_constraint = props.proposed_addition.constraint.getCurrentData();
	}

	const request: ASRequest = {
		action: Command_Actions.new_prefix,
		context: Command_Contexts.staging,
		parameters: {
			[Command_Parameters.data]: data
		}
	};

	props.app.awaiting_server_response.set(true);
	sendPOST(
		request,
		stagingModificationResponseHandlerBuilder(props),
		props.app.notAuthorizedHandler,
		props.app.errorHandler
	);
};

const deleteFromStaging = (props: EditBoxProps) => {
	console.debug('DeleteFromStaging ', props);

	props.app.getWorkTracker().clear();

	const data: number[] = [] as number[];
	if (props.selection !== null) {
		props.selection.forEach((v) => {
			data.push(v);
		});
	}

	const request: ASRequest = {
		action: Command_Actions.delete_prefix,
		context: Command_Contexts.staging,
		parameters: {
			[Command_Parameters.data]: data
		}
	};

	props.app.awaiting_server_response.set(true);
	sendPOST(
		request,
		stagingModificationResponseHandlerBuilder(props),
		props.app.notAuthorizedHandler,
		props.app.errorHandler
	);
};

const stagingCommit = (props: CommitBoxProps) => {
	const request: ASRequest = {
		action: Command_Actions.initiate_prefix,
		context: Command_Contexts.staging,
		parameters: {
			[Command_Parameters.data]: 'commit'
		}
	};

	props.app.awaiting_server_response.set(true);
	const response_handler = (response: OperationContents) => {
		props.app.updateDisplay(response);
	};
	sendPOST(request, response_handler, props.app.notAuthorizedHandler, props.app.errorHandler);
};

export const AddPanel: FC<AddBoxProps> = (props: AddBoxProps) => {
	//const new_constraint = new WrappedHook<GenericRenderedConstraint>(null);

	console.debug('AddPanel render', props);
	const contextChange = (evt: any) => {
		console.debug('Context change');
		const newcontext = parseInt(evt.target.value);
		const proposal: ProposedAddition = {
			//selected_date:props.proposed_addition.selected_date,
			context: newcontext,
			selected_type: null,
			multiple: 1
		};
		props.interaction_handler(Interaction.changeProposedAddition, proposal);
	};

	let tsb_value = null;
	if (
		props.proposed_addition.selected_type !== null &&
		props.proposed_addition.selected_type !== undefined
	) {
		tsb_value = props.proposed_addition.selected_type.toString();
	}

	const typeSelectChange = (evt: any, new_type_string: string) => {
		let new_type = parseInt(new_type_string);
		if (isNaN(new_type)) {
			new_type = null;
		}

		const new_proposal: ProposedAddition = {
			context: props.proposed_addition.context,
			selected_type: new_type,
			multiple: props.proposed_addition.multiple
		};

		switch (props.proposed_addition.context) {
			case AddType.Assignment:
				break;
			case AddType.ScheduleTemplate:
				break;
			case AddType.Constraint:
				{
					const cc: ConstraintClass = new_type as ConstraintClass;

					if (new_type !== null) {
						console.debug('Context is constraint.', tsb_value, cc);
						new_proposal.constraint = createNewRenderedConstraint(cc, props.staging_data);
						new_proposal.selected_type = cc;
					}
				}
				break;
			default:
				console.debug('Unhandled type select.');
		}

		props.interaction_handler(Interaction.changeProposedAddition, new_proposal);
	};

	let constraint_manipulate: React.ReactNode = null;
	if (
		props.proposed_addition.context == AddType.Constraint &&
		props.proposed_addition.constraint !== undefined
	) {
		constraint_manipulate = props.proposed_addition.constraint.createFormComponent(
			props.app,
			props.staging_data,
			props.interaction_handler,
			false
		);
	}

	const dateSelectChange = (new_date: Date) => {
		console.debug('Date select changed to ', new_date);
		props.interaction_handler(Interaction.selectDate, { date: new_date, multi: false });
	};

	const multipleChange = (e) => {
		const new_proposal = {} as ProposedAddition;
		Object.assign(new_proposal, props.proposed_addition);
		new_proposal.multiple = parseInt(e.target.value);
		props.interaction_handler(Interaction.changeProposedAddition, new_proposal);
	};

	let selection_options: number[];
	let selection_option_labels: { [i: number]: string };

	switch (props.proposed_addition.context) {
		case AddType.Assignment:
			selection_options = props.staging_data.assignment_types.getIDs();
			selection_option_labels = props.staging_data.assignment_types.getLabels();
			break;
		case AddType.ScheduleTemplate:
			selection_options = props.staging_data.schedule_template_types.getIDs();
			selection_option_labels = props.staging_data.schedule_template_types.getLabels();
			break;
		case AddType.Constraint:
			selection_options = [];
			selection_options.push(ConstraintClass.SingleWorker);
			selection_options.push(ConstraintClass.MatchOne);
			selection_option_labels = {};
			selection_option_labels[ConstraintClass.SingleWorker] = 'Single Worker';
			selection_option_labels[ConstraintClass.MatchOne] = 'Match One';
			break;
		default:
			selection_options = [];
			selection_option_labels = {};
	}

	let date_select = null;
	let number_select = null;
	if (props.proposed_addition.context !== AddType.Constraint) {
		console.debug('Creating date picker', props.selected_date, dateSelectChange);
		date_select = (
			<LinkedDatePicker
				id="staging_date"
				disabled={false}
				label="Date"
				date={props.selected_date}
				handler={dateSelectChange}
			/>
		);
		number_select = (
			<TextField
				style={{ marginTop: 10 }}
				id="staging_number"
				disabled={false}
				label="Multiple"
				type="number"
				value={props.proposed_addition.multiple}
				onChange={multipleChange}
				inputProps={{ inputMode: 'numeric', pattern: '[0-9]*' }}
			/>
		);
	}

	const type_selection_enabled = selection_options.length > 0;

	//console.debug("Autocomplete",type_selection_options,tsb_value);

	const type_selection_box = (
		<Autocomplete
			autoSelect
			id="type_select"
			disabled={!type_selection_enabled}
			value={tsb_value}
			onChange={typeSelectChange}
			options={selection_options}
			getOptionLabel={(option) => {
				const o = selection_option_labels[option];
				if (o === undefined) {
					return '';
				} else {
					return o;
				}
			}}
			isOptionEqualToValue={(option, value) => {
				return option == value;
			}}
			style={{
				marginTop: '20px',
				marginBottom: '20px',
				width: panel_style.width
			}}
			renderInput={(params) => <TextField {...params} label="Select Type" variant="outlined" />}
		/>
	);

	console.debug('Constraint Manipulate', constraint_manipulate);

	/*
    let add_button_disabled:boolean = (
        props.proposed_addition.selection===null || //Selection must never be null or undefined
        props.proposed_addition.selection===undefined
        || 
        (
            //If the proposed addition is a assignment or schedule template, disable the add button if the selected date isn't defined.
            (
                props.proposed_addition.context==AddType.Assignment ||
                props.proposed_addition.context==AddType.ScheduleTemplate
            )
            &&
            props.selected_date===null || 
            props.selected_date===undefined
        ) 
        ||
        (
            //If the proposed addition is a constraint and that constraint isn't valid, disable the box.
            props.proposed_addition.context==AddType.Constraint
            &&
            (
                props.proposed_addition.constraint!==undefined &&
                !props.proposed_addition.constraint.validate()
            )
        )
    );
    */

	const getAddButtonDisabled = () => {
		//Disable add box if type isn't selected
		if (
			props.proposed_addition.selected_type === null ||
			props.proposed_addition.selected_type === undefined
		) {
			return true;
		}
		switch (props.proposed_addition.context) {
			case AddType.Assignment:
			case AddType.ScheduleTemplate:
				//If the proposed addition is a assignment or schedule template, disable the add button if the selected date isn't defined.
				return props.selected_date === null || props.selected_date == undefined;
			case AddType.Constraint:
				//If the proposed addition is a constraint and that constraint isn't valid, disable the box.
				return (
					props.proposed_addition.constraint !== undefined &&
					props.proposed_addition.constraint !== null &&
					!props.proposed_addition.constraint.validate()
				);
			default:
				return true;
		}
	};

	return (
		<Box id="add_panel" style={panel_style}>
			<FormControl component="fieldset" sx={{ width: '100%' }}>
				<FormLabel>Staging Context</FormLabel>
				<RadioGroup
					id="context"
					aria-label="context"
					name="context"
					value={props.proposed_addition.context}
					onChange={contextChange}
				>
					<FormControlLabel
						id="sa"
						value={AddType.Assignment}
						control={<Radio />}
						label="Single Assignment"
					/>
					<FormControlLabel
						id="sg"
						value={AddType.ScheduleTemplate}
						control={<Radio />}
						label="Schedule Template"
					/>
					<FormControlLabel
						id="const"
						value={AddType.Constraint}
						control={<Radio />}
						label="Constraint"
					/>
				</RadioGroup>
				{type_selection_box}
				{constraint_manipulate}
				{date_select}
				{number_select}
				<Button
					id="addtostaging"
					style={{ marginTop: 15 }}
					variant="contained"
					color="primary"
					onClick={() => {
						addToStaging(props);
					}}
					disabled={getAddButtonDisabled()}
				>
					<Typography>Add</Typography>
				</Button>
			</FormControl>
		</Box>
	);
};

export const EditPanel: FC<EditBoxProps> = (props: EditBoxProps) => {
	console.debug('Edit panel', props);

	let edit_individual_staging_item = null;
	let assignables_only_portion = null;
	let delete_button = null;

	if (props.selection != null && props.selection.size > 0) {
		if (props.selection.size == 1) {
			props.selection.forEach((selected_index) => {
				const selected_rendered_assignable =
					props.staging_data.rendered_assignables[selected_index];
				if (selected_rendered_assignable !== null && selected_rendered_assignable !== undefined) {
					const afc_props: AssignableFormComponentProps = {
						app: props.app,
						selection: props.selection,
						update_data: props.update_data,
						modificationResponseHandler: stagingModificationResponseHandlerBuilder(props),
						rendered_assignable: selected_rendered_assignable
					};
					edit_individual_staging_item = AssignableFormComponent(afc_props);
				}

				//Try as a constraint
				const selected_rendered_constraint =
					props.staging_data.rendered_constraints[selected_index];
				if (selected_rendered_constraint !== null && selected_rendered_constraint !== undefined) {
					const modify = () => {
						selected_rendered_constraint.sendModificationRequest(
							props.app,
							stagingModificationResponseHandlerBuilder(props)
						);
						props.interaction_handler(Interaction.releaseFocusedSelection);
						props.interaction_handler(Interaction.clearPrimarySelection);
					};

					edit_individual_staging_item = (
						<Box width="100%">
							{selected_rendered_constraint.createFormComponent(
								props.app,
								props.staging_data,
								props.interaction_handler,
								true
							)}
							<Button
								style={{ width: '100%', marginTop: '15px' }}
								aria-label="discard"
								color="primary"
								variant="contained"
								onClick={modify}
								disabled={
									!selected_rendered_constraint.hasProposedDetails() ||
									!selected_rendered_constraint.validate()
								}
							>
								Save Changes
							</Button>
						</Box>
					);
				}
			});
		}
		let contains_no_constraints = true;
		for (const selected_index of Array.from(props.selection)) {
			const selected_rendered_constraint = props.staging_data.rendered_constraints[selected_index];
			if (selected_rendered_constraint !== null && selected_rendered_constraint !== undefined) {
				contains_no_constraints = false;
				break;
			}
		}
		if (contains_no_constraints) {
			const unassign = () => {
				const parameters = {} as ASRequestStagingParameters;
				parameters.type = Staging_Directives.unassign;
				parameters.staging_ids = Array.from(props.selection);
				console.debug('Unassigning with parameters:', parameters);
				props.app.singleModify(parameters, stagingModificationResponseHandlerBuilder(props));
			};

			const changeLock = (lock: boolean) => {
				const parameters = {} as ASRequestStagingParameters;
				if (lock) {
					parameters.type = Staging_Directives.lock;
				} else {
					parameters.type = Staging_Directives.unlock;
				}
				parameters.staging_ids = Array.from(props.selection);
				console.debug('Lock change:', parameters);
				props.app.singleModify(parameters, stagingModificationResponseHandlerBuilder(props));
			};

			assignables_only_portion = (
				<Grid
					width="100%"
					key="assignables_only"
					container
					direction="column"
					display="flex"
					justifyContent="space-between"
					alignItems="center"
				>
					<Grid
						container
						width="100%"
						direction="row"
						item
						xs={12}
						justifyContent="space-around"
						alignItems="center"
					>
						<Grid item xs="auto">
							<IconButton
								id="lock"
								key="ico1"
								color="primary"
								onClick={() => {
									changeLock(true);
								}}
							>
								<LockIcon />
							</IconButton>
						</Grid>
						<Grid item xs="auto">
							<IconButton
								id="unlock"
								key="ico2"
								color="primary"
								onClick={() => {
									changeLock(false);
								}}
							>
								<LockOpenIcon />
							</IconButton>
						</Grid>
					</Grid>
					<Grid item sx={{ width: '100%' }}>
						<Button
							id="unassign"
							key="button1"
							sx={{ width: '100%' }}
							variant="contained"
							color="primary"
							onClick={unassign}
						>
							<Typography>Unassign</Typography>
						</Button>
					</Grid>
				</Grid>
			);
		}
		const deleteSelection = () => {
			const confirm_function = () => {
				deleteFromStaging(props);
			};

			let selected_assignable_count = 0;
			let selected_constraint_count = 0;

			props.selection.forEach((i: number) => {
				const as_ra = props.staging_data.rendered_assignables[i];
				const as_rc = props.staging_data.rendered_constraints[i];
				if (as_ra !== null && as_ra !== undefined) {
					selected_assignable_count++;
				} else if (as_rc !== null && as_rc !== undefined) {
					selected_constraint_count++;
				}
			});

			let message = '';
			if (selected_assignable_count > 0) {
				message += selected_assignable_count + ' assignable';
				if (selected_assignable_count > 1) {
					message += 's';
				}
				if (selected_constraint_count > 0) {
					message += ' andtype_selection_box ';
				}
			}
			if (selected_constraint_count > 0) {
				message += selected_constraint_count + ' constraint';
				if (selected_constraint_count > 1) {
					message += 's';
				}
			}
			if (selected_assignable_count + selected_constraint_count > 1) {
				message += ' are';
			} else {
				message += ' is';
			}
			message += ' selected for deletion. Are you sure you want to proceed? This cannot be undone.';
			const cdprops: ConfirmDialogProps = {
				app: props.app,
				title: 'Confirm Deletion',
				message: message,
				confirm_button_text: 'Delete',
				confirm_function: confirm_function
			};

			const confirm_dialog = ConfirmDialog(cdprops);

			props.app.showSuperDialog(confirm_dialog);
		};

		delete_button = (
			<Button
				id="delete_selection"
				key="button2"
				style={{ width: '100%', marginTop: '15px' }}
				variant="contained"
				color="error"
				onClick={deleteSelection}
			>
				<Typography>Delete</Typography>
			</Button>
		);
	}

	return (
		<Box width="100%" key="edit_box" style={panel_style}>
			<FormControl key="formcontrol" component="fieldset" sx={{ width: '100%', height: '100%' }}>
				<Grid container width="100%" direction="column" justifyContent="space-between" flexGrow="1">
					<Grid item width="100%">
						{edit_individual_staging_item}
						{assignables_only_portion}
					</Grid>
					<Grid item>{delete_button}</Grid>
				</Grid>
			</FormControl>
		</Box>
	);
};

export const CommitPanel: FC<CommitBoxProps> = (props: CommitBoxProps) => {
	console.debug('Commit panel', props);
	const info_message_style = { marginTop: '10px' };
	const info_message = [
		<Typography key="message1" style={info_message_style}>
			Assigned, locked assignments prior to today's date that are not entailed with the schedule may
			be committed.{' '}
		</Typography>,
		<Typography key="message2" style={info_message_style}>
			There are {props.update_data.commitable.length} commitable items.
		</Typography>
	];

	if (props.update_data.commitable.length > 0) {
		info_message.push(
			<Typography key="message3" style={info_message_style}>
				These are highlighted in the table.
			</Typography>
		);
	}

	const commit = () => {
		const commit_message = 'Are you sure you want to commit? This cannot be undone.';
		const cdprops: ConfirmDialogProps = {
			app: props.app,
			title: 'Confirm Commit',
			message: commit_message,
			confirm_button_text: 'Commit',
			confirm_function: () => {
				stagingCommit(props);
			}
		};

		const confirm_dialog = ConfirmDialog(cdprops);

		props.app.showSuperDialog(confirm_dialog);
	};

	const commit_button = (
		<Button
			id="delete_selection"
			key="button2"
			style={{ width: '100%', marginTop: '15px' }}
			variant="contained"
			color="primary"
			onClick={commit}
		>
			<Typography>Commit</Typography>
		</Button>
	);

	return (
		<Box width="100%" key="commit_box" style={panel_style}>
			<FormControl key="formcontrol" component="fieldset" sx={{ width: '100%', height: '100%' }}>
				<Grid container width="100%" direction="column" justifyContent="sstart" flexGrow="1">
					<Grid item width="100%" justifyContent="left">
						{info_message}
					</Grid>
					<Grid item width="100%" justifyContent="center">
						{commit_button}
					</Grid>
				</Grid>
			</FormControl>
		</Box>
	);
};
