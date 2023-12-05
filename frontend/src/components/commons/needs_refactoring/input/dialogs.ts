import type { ASDisplayResponseData } from '../../ancillary/ajax/commands_generic';
import type { AutoschedaState } from '../../refactored/autoscheda_core';
import type { Display_Main_Props } from '../displays/display';

export interface DialogBaseProps {
	app: AutoschedaState;
	title: string;
	content: any;
}

/*
<TextField
    autoFocus
    margin="dense"
    label="Email Address"
    type="email"
    fullWidth
/>
*/

export function ShowElementDialog_Base(
	props: DialogBaseProps,
	remove_button: any,
	savehandler: null | ((evt: any) => void)
): any {
	const close_dialog = (evt: any) => props.app.showDialog(null);
	const save_dialog = (evt: any) => {
		close_dialog(evt);
		if (savehandler !== null) {
			savehandler(evt);
		}
	};
	/*
    return (
        <Box>
            <Dialog open={true} onClose={close_dialog} fullWidth={true} maxWidth={false} disableEscapeKeyDown={true} fullScreen={true} aria-labelledby="form-dialog-title">
                <DialogTitle key={"title"}>{props.title}</DialogTitle>
                <DialogContent key={"content"}>
                    {props.content}
                </DialogContent>
                <DialogActions key={"actions"} style={{borderTopStyle:"solid", borderTopWidth:1, borderTopColor:autoscheda_theme.palette.common.white}}>
                    {remove_button}
                    <Box flexGrow={1} flexShrink={1} />                        
                    <Button id={"save_button"} key={"save_button"} onClick={save_dialog} color="primary" variant="contained">
                        Save
                    </Button>
                    <Button id={"discard_button"} key={"discard_button"} onClick={close_dialog} color="secondary" variant="contained">
                        Discard Changes
                    </Button>
                </DialogActions>
            </Dialog>
        </Box>
       // TODO
    );*/
	return null;
}

export interface InfoDialogProps {
	app: AutoschedaState;
	title: string;
	message: string;
	confirm_button_text: string;
}

export interface ConfirmDialogProps extends InfoDialogProps {
	confirm_function: (evt: any) => void;
}

export interface RawEditDialogProps {
	display_props: Display_Main_Props;
	row_key: string;
}

export const BuildDialogBaseProps = (
	title: string,
	data_object: {},
	update_data: ASDisplayResponseData,
	app: AutoschedaState
): DialogBaseProps => {
	const baseprops: DialogBaseProps = {
		app: app,
		title: title,
		content: null
	};

	for (const colkey in update_data.cols) {
		const col = update_data.cols[colkey];

		if (!col.hidden) {
			const value = data_object[colkey];
			const datatype: DataType = col.data_type as DataType;

			const umeta = app.current_response.get().user_meta;
			const col_is_readonly = !userHasAccess(umeta, col.access);

			console.debug('Column is readonly: ', col_is_readonly, umeta, col.access);

			switch (datatype) {
				case DataType.Binary:
					const bn: BinaryNode = new BinaryNode(
						update_data.meta,
						update_data.cols[colkey].meta,
						value
					);
					baseprops.content.push(bn.createRootInputNode());
					break;
				case DataType.Enum:
					{
						let order: (number | string)[];
						if (col.meta.exclusive) {
							order = [];
							for (const i of col.meta.labels.order) {
								order.push(i);
							}
							console.debug('Exclusive column.');
							//This column's value must be unique for each row in the table unless null.
							for (const row_index in update_data.rows) {
								const this_cell_value = update_data.rows[row_index][colkey];
								console.debug('Checking row index ' + row_index, this_cell_value, value);
								if (this_cell_value && this_cell_value != value) {
									console.debug('Removing value ' + this_cell_value);
									for (const o_index in order) {
										const i_as_number = parseInt(o_index);
										if (order[o_index] == this_cell_value) {
											console.debug(
												'Removing o_index ',
												i_as_number,
												order,
												order.slice(0, i_as_number),
												order.slice(i_as_number + 1, order.length)
											);
											order = order
												.slice(0, i_as_number)
												.concat(order.slice(i_as_number + 1, order.length));
											console.debug(order);
										}
									}
								}
							}
						} else {
							order = col.meta.labels.order;
						}
						baseprops.content.push(
							PickList.create(
								col.name,
								col.name,
								col_is_readonly,
								data_object,
								colkey,
								col.meta.labels.map,
								undefined,
								col.meta.nullable,
								order,
								col.meta.labels.disabled
							)
						);
					}
					break;
				case DataType.EnumArray:
					baseprops.content.push(
						createValueBackedMultiplePickList(
							col.name,
							col_is_readonly,
							data_object,
							colkey,
							col.meta.labels.map,
							undefined,
							col.meta.labels.order
						)
					);
					break;
				default:
					baseprops.content.push(
						GenericInputField(
							col.name,
							col.name,
							col.data_type,
							col_is_readonly,
							data_object,
							colkey,
							undefined
						)
					);
			}
		}
	}

	return baseprops;
};

export function RetireButton(app: AutoschedaState, handler: (evt: any) => void): any {
	const props: ConfirmDialogProps = {
		app: app,
		title: 'Confirm Retire',
		message: 'Are you sure you want to retire this item?',
		confirm_button_text: 'Retire',
		confirm_function: handler
	};
	const confirm_dialog = ConfirmDialog(props);

	const confirm_func = (evt: any) => props.app.showSuperDialog(confirm_dialog);
	/*
    return (
    <Button id={"retire_button"} key={"retire_button"} onClick={confirm_func} color="error" variant="contained">
        Retire
    </Button>
    );
    */
	// TODO
	return null;
}

export function DeleteButton(app: AutoschedaState, function_to_confirm: (evt: any) => void): any {
	const props: ConfirmDialogProps = {
		app: app,
		title: 'Confirm Deletion',
		message: 'Are you sure you want to delete this item? This cannot be undone.',
		confirm_button_text: 'Delete',
		confirm_function: function_to_confirm
	};

	const confirm_dialog = ConfirmDialog(props);

	const confirm_func = (evt: any) => props.app.showSuperDialog(confirm_dialog);

	/*
    return (
    <Button id={"delete_button"} key={"delete_button"} onClick={confirm_func} color="error" variant="contained">
        Delete
    </Button>
    );
    */
	//TODO
	return null;
}

export function InfoDialog(props: InfoDialogProps): any {
	const ack_func = (evt: any) => props.app.showSuperDialog(null);
	/*
    return (
        <div>
            <Dialog open={true} onClose={ack_func} fullWidth={false} maxWidth={false} disableEscapeKeyDown={true} fullScreen={false} aria-labelledby="form-dialog-title">
                <DialogTitle key={"title"}>{props.title}</DialogTitle>
                <DialogContent key={"content"}>
                    {props.message}
                </DialogContent>
                <DialogActions key={"actions"}>
                    <Button id={"confirm_button"} key={"confirm_button"} onClick={ack_func} color="primary" variant="contained">
                        {props.confirm_button_text}
                    </Button>
                </DialogActions>
            </Dialog>
        </div>
    );
    */
	// TODO
	return null;
}

export function ConfirmDialog(props: ConfirmDialogProps): any {
	const cancel_func = (evt: any) => props.app.showSuperDialog(null);
	const confirm_func = (evt: any) => {
		props.app.showSuperDialog(null);
		props.app.showDialog(null);
		props.confirm_function(evt);
	};
	/*
    return (
        <Paper elevation={6}>
            <Dialog open={true} onClose={cancel_func} fullWidth={false} maxWidth={false} disableEscapeKeyDown={true} fullScreen={false} aria-labelledby="form-dialog-title">
                <DialogTitle key={"title"}>{props.title}</DialogTitle>
                <DialogContent key={"content"}>
                    <Typography variant="larger">{props.message}</Typography>
                </DialogContent>
                <DialogActions key={"actions"}>
                    <Button id={"confirm_button"} key={"confirm_button"} onClick={confirm_func} color="primary" variant="contained">
                        {props.confirm_button_text}
                    </Button>
                    <Box flexGrow={1} flexShrink={1} />                        
                    <Button id={"cancel_button"} key={"cancel_button"} onClick={cancel_func} color="secondary" variant="contained">
                        Cancel
                    </Button>
                </DialogActions>
            </Dialog>
        </Paper>
    );
    */
	// TODO
	return null;
}

export function ShowRawTableEditDialog(props: RawEditDialogProps): any {
	const original_display_data: DisplayOperationContents = props.display_props
		.response as DisplayOperationContents;

	console.debug('rawtableeditdiaglog Parsing JSON', original_display_data);
	const cloned_display_data: DisplayOperationContents = JSON.parse(
		JSON.stringify(original_display_data)
	);
	const update_data: ASDisplayResponseData = cloned_display_data.update_data; //Create a clone for the input to manipulate to allow saving/discarding changes.
	const row = update_data.rows[props.row_key];

	const baseprops = BuildDialogBaseProps(
		'Modify Element',
		row,
		update_data,
		props.display_props.app
	);

	const save_handler = (evt: any) => {
		console.debug('Save handler called.');
		props.display_props.app.commitWholeRowModify(
			original_display_data,
			cloned_display_data,
			props.row_key
		);
	};

	const delete_handler = (evt: any) => {
		console.debug('Delete/Retire handler called.');
		let show_retired = (props.display_props.request.parameters as AutoschedaDisplayParameters)
			.show_retired;
		if (show_retired !== undefined && show_retired == true) {
			console.debug(
				"Didn't delete this row.",
				props.row_key,
				show_retired,
				props.display_props.request.parameters
			);
		} else {
			delete cloned_display_data.update_data.rows[props.row_key];
			console.debug('Deleted this row!', props.row_key, cloned_display_data);
		}
		props.display_props.app.deleteMember(original_display_data, cloned_display_data, props.row_key);
	};

	let button = null;
	if (original_display_data.update_data.meta.retireable) {
		button = RetireButton(props.display_props.app, delete_handler);
	} else {
		button = DeleteButton(props.display_props.app, delete_handler);
	}

	return ShowElementDialog_Base(baseprops, button, save_handler);
}

export function ShowRawTableNewDialog(props: Display_Main_Props): void {
	processInstantiator(props);
}

export function ShowRawTableCopyDialog(props: Display_Main_Props, original_row: {}): void {
	console.debug('Building copy item dialog.', props, original_row);

	const update_data: ASDisplayResponseData = props.response.update_data as ASDisplayResponseData;
	const new_row = {};
	for (const colkey in update_data.cols) {
		const col = update_data.cols[colkey];
		if (original_row !== undefined) {
			console.debug('rawtablecopydialog JSON parse', original_row[colkey]);
			new_row[colkey] = JSON.parse(JSON.stringify(original_row[colkey]));
		} else {
			new_row[colkey] = {};
		}
	}

	processInstantiator(props, new_row);
}

export function GenericInputField(
	id: string,
	label: string,
	data_type: DataType,
	readonly: boolean,
	value_object: { [i: string]: any },
	value_key: string,
	changeHandler: undefined | (() => void)
): any {
	console.debug(
		'Data type for ' + label + ' is ' + data_type.toString() + ' with value object ',
		value_object
	);
	if (value_object[value_key] === undefined) {
		console.debug('Initial value is unefined.');
		console.trace();
	}
	switch (data_type) {
		case DataType.Boolean:
			return createBooleanControl(id, label, readonly, value_object, value_key, changeHandler);
		case DataType.Float:
			return createFloatControl(id, label, readonly, value_object, value_key, changeHandler);
		case DataType.DisableableDouble:
			return (
				<DisableableDoubleControl
					id={id}
					label={label}
					disabled={readonly}
					object={value_object}
					object_key={value_key}
					changeHandler={changeHandler}
				/>
			);
		case DataType.Integer:
		case DataType.Long:
		case DataType.Enum:
		case DataType.NodeReference:
			return createIntegerControl(id, label, readonly, value_object, value_key, changeHandler);
		case DataType.LocalDate:
			return createDateStringControl(id, label, readonly, value_object, value_key, changeHandler);
		case DataType.LocalDateAsEpochDay:
			return createEpochDayControl(id, label, readonly, value_object, value_key, changeHandler);
		case DataType.LocalTime:
			return createGenericControl<string>(
				id,
				label,
				'time',
				readonly,
				value_object,
				value_key,
				changeHandler
			);
		case DataType.String:
			return createGenericControl<string>(
				id,
				label,
				'text',
				readonly,
				value_object,
				value_key,
				changeHandler
			);
		default:
			console.error('Unhandled input type ' + data_type + ', defaulting to text field.');
			return createGenericControl<string>(
				id,
				label,
				'text',
				readonly,
				value_object,
				value_key,
				changeHandler
			);
	}
}
