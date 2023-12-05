import type {
	AutoschedaDisplayParameter,
	AutoschedaDisplayParameters,
	AutoschedaState
} from '../../needs_refactoring/autoscheda_core';
import type { DataMeta, DataType } from '../../needs_refactoring/data_processing/data_types';
import type { KeyedTable_Data } from '../../needs_refactoring/displays/db/keyed';
import type { ConfirmDialogProps } from '../../needs_refactoring/input/dialogs';
import { CursorRelevantState, setCursorRelevantState } from '../commons/DOMfunctions';
import { csrf_token_header, getCSRFToken, getDOMCSRFFormInput } from '../csrf';
import type { AssignmentTypeRow } from '../extended_types/id_tables/AssignmentTypeTable';
import type {
	ASStagingResponseMessage,
	ConstraintStagingData
} from '../staging/data_processing/stagingdata';

export interface ASRequest {
	action: Command_Actions;
	context: Command_Contexts;
	parameters: ASRequestParameters;
}

export enum Command_Actions {
	new_prefix = 'new',
	modify_prefix = 'modify',
	bulk_modify_prefix = 'bulk_modify',
	delete_prefix = 'delete',
	initiate_prefix = 'initiate',
	view_prefix = 'view'
}

export enum Command_Contexts {
	my_schedule = 'showmyschedule',
	schedule_byassignment = 'schedulebyassignment',
	schedule_byworker = 'schedulebyworker',
	assignment_types = 'assignment_types',
	worker_list = 'worker_list',
	schedule_template_types = 'schedule_template_types',
	web_users = 'web_users',
	staging = 'staging',
	individual_requests = 'individual_requests',
	global_requests = 'global_requests',
	solve = 'solve',
	tracking_totals = 'tracking_totals',
	tracking_adjustments = 'tracking_adjustments',

	responsibility = 'worker_responsibility',
	//respon_snapshot="worker_responsibility_snapshot",

	user_password = 'user_password'
}

export interface CommandParameters {
	old?:any,
	new?:any,
	data?:any
}

export enum Staging_Directives {
	assign = 'assign',
	unassign = 'unassign',
	modify_constraint = 'modify_constraint',
	lock = 'lock',
	unlock = 'unlock'
}

export type ResponseHandler = (response: any) => void;

export enum Authorization {
	User = 'ROLE_USER',
	Scheduler = 'ROLE_SCHEDULER',
	Admin = 'ROLE_ADMIN'
}
//export type Authorization = "ROLE_USER"|"ROLE_SCHEDULER"|"ROLE_ADMIN";
export enum ColumnAccess {
	User = 0,
	Scheduler = 1,
	Admin = 2,
	System = 3
}

export const userHasAccess = (user_auth_level: UserMeta, access: ColumnAccess): boolean => {
	switch (access) {
		case ColumnAccess.User:
			return true;
		case ColumnAccess.Scheduler: {
			for (const auth of user_auth_level.auth) {
				if (auth === Authorization.Scheduler || auth === Authorization.Admin) {
					return true;
				}
			}
			return false;
		}
		case ColumnAccess.Admin: {
			for (const auth of user_auth_level.auth) {
				if (auth === Authorization.Admin) {
					return true;
				}
			}
			return false;
		}
		case ColumnAccess.System:
		default:
			return false;
	}
};

export interface Column {
	name: string;
	data_type?: DataType;
	meta: DataMeta;
	access: ColumnAccess;
	hidden: boolean;
}

export const getValidOrderedColumnIndices = (column: Column) => {
	const retval = { by_member_id: {}, by_order: {} } as {
		by_member_id: { [i: number]: number }; //key is index of member, value is order index
		by_order: { [i: number]: number }; //key is order index, value is index of member
	};

	let current_index = 0;
	for (const next_id of column.meta.labels.order) {
		//Check whether this index is disabled
		if (!(column.meta.labels.disabled && column.meta.labels.disabled.indexOf(next_id) >= 0)) {
			retval.by_member_id[next_id] = current_index;
			retval.by_order[current_index] = next_id;
			current_index++;
		}
	}

	return retval;
};

export type FilterMembers = (string | boolean)[];
export type Filter = { [colkey: string]: FilterMembers };

export type ASRequestParameters =
	| ASRequestStagingParameters
	| AutoschedaDisplayParameters
	| ASRequestDataCommand
	| { password: string }
	| null;

export async function sendPOST(
	request: ASRequest,
	successHandler: ResponseHandler = defaultResponseHandler,
	notAllowedHandler: () => any,
	errorHandler: (message: string) => any
) {
	setCursorRelevantState(CursorRelevantState.WaitingOnServerResponse, true);

	const headers = {
		['Content-Type']: 'application/json',
		[csrf_token_header]: getCSRFToken()
	};

	const req_init: RequestInit = {
		method: 'POST',
		credentials: 'include',
		body: JSON.stringify(request)
	};
	req_init.headers = headers;

	const endpoint=import.meta.env.VITE_POST_ROOT+'/command';
	console.debug('Sending request.', endpoint, req_init);
	try {
		const response = await fetch(endpoint, req_init);
		console.debug('Response received.', response);
		if (response.ok) {
			try {
				const parsed: OperationResult = await response.json();
				console.debug('Response parsed.', parsed);
				if (isDeauth(parsed)) {
					console.debug('Not allowed');
					notAllowedHandler();
				} else if (isFailure(parsed)) {
					let message = 'Error returned';
					if (parsed.message) {
						message += ': ' + parsed.message;
					}
					errorHandler(message);
				} else {
					successHandler(parsed.contents);
				}
			} catch (error) {
				console.error('Parse error.', error);
			}
		} else {
			console.debug('Response is not okay. Status is ', response.status);
			switch (response.status) {
				case 401: //deauthenticated
					console.debug('Case 401 handler.');
					window.location.href = '/login';
					break;

				default: //unhandled
					console.debug('Default handler.');
					errorHandler(response.statusText);
					break;
			}
		}
		setCursorRelevantState(CursorRelevantState.WaitingOnServerResponse, false);
	} catch (error) {
		console.error('Fetch error.', error);
		setCursorRelevantState(CursorRelevantState.WaitingOnServerResponse, false);
	}
}

const isDeauth = function (response: OperationResult): boolean {
	return response.not_allowed !== undefined && response.not_allowed;
};

const isFailure = function (response: OperationResult): boolean {
	return response.success === undefined || response.success === null || !response.success;
};

interface OperationResult {
	success: boolean | undefined;
	message: string | undefined;
	not_allowed: boolean | undefined;
	contents: OperationContents | undefined;
}

export enum UpdateTypes {
	raw = 'raw',
	keyed = 'keyed',
	simple = 'simple',
	display_meta = 'display_meta',
	staging = 'staging',
	solve = 'solve',
	responsibility = 'responsibility'
}

export type ASRequestStagingParameters = {
	type: string;
	worker_id?: number;
	staging_ids?: number[];
	staging_id: number;
	data: ConstraintStagingData;
};

export type ASRequestDataCommand = {
	data?: any,
	updates?: any,
	deletions?: any,
	messages?: string[]
};

export interface UserMeta {
	auth: Authorization[];
}

export interface OperationContents {
	update_type: UpdateTypes;
	update_data: ASDisplayResponseData | ASStagingResponseMessage | KeyedTable_Data; //Could do a better job with these types; KeyedTable_Data is really a child of ASDisplayResponseData
	user_meta: UserMeta;
}

export interface DisplayOperationContents extends OperationContents {
	update_data: ASDisplayResponseData;
}

export interface StagingOperationContents extends OperationContents {
	update_data: ASStagingResponseMessage;
}

export type GenericRow = { [row_index: number]: { [column_index: number]: any } };
export interface ASDisplayResponseData {
	headers?: {};
	cols: { [key: string]: Column };
	rows: GenericRow | AssignmentTypeRow;
	membermap: {} | undefined;
	meta: DataMeta;
}

const defaultResponseHandler: ResponseHandler = function (response: any) {
	console.debug('Default response handler.', response);
};

//export const ISO_date_format_moment = "YYYY-MM-DD";

const datespan_elements: AutoschedaDisplayParameter[] = ['start_date', 'end_date'];
const date_elements: AutoschedaDisplayParameter[] = ['date'];
const retireable_elements: AutoschedaDisplayParameter[] = ['show_retired'];

export const checkForUnsavedWorkBeforePerformingFunction = (
	app: AutoschedaState,
	func: () => void
) => {
	const func_with_clear = () => {
		func();
		app.getWorkTracker().clear();
	};

	if (app.getWorkTracker().unsavedUserWork()) {
		console.debug('Checking whether user wants to discard work.');
		const cdprops: ConfirmDialogProps = {
			app: app,
			title: 'Unsaved Work',
			message:
				'There are unsaved changes. Are you sure you wish to navigate away? These changes will be lost.',
			confirm_button_text: 'Continue',
			confirm_function: func_with_clear
		};

		//const confirm_dialog = ConfirmDialog(cdprops);

		app.showSuperDialog(null);
	} else {
		console.debug('No unsaved work recorded.');
		func_with_clear();
	}
};

export const logout = (app: AutoschedaState) => {
	const func = () => {
		const logouturl = '/logout';
		const form = document.createElement('form');
		form.method = 'post';
		form.action = logouturl;
		const csrf_input = getDOMCSRFFormInput();
		form.appendChild(csrf_input);
		console.debug('Created logout form');
		document.body.appendChild(form);
		console.debug('Submitting.');
		form.submit();
	};
	checkForUnsavedWorkBeforePerformingFunction(app, func);
};

export const rawnavigate = (app: AutoschedaState, url: string) => {
	const func = () => {
		window.location.assign(url);
	};
	checkForUnsavedWorkBeforePerformingFunction(app, func);
};

export const requestMySchedule = function (app: AutoschedaState) {
	//const func=()=>{ //This check is done by app.changeDisplayContext
	app.changeDisplayContext(
		Command_Actions.view_prefix,
		Command_Contexts.my_schedule,
		datespan_elements
	);
};

export const requestScheduleByWorker = function (app: AutoschedaState) {
	//const func=()=>{ //This check is done by app.changeDisplayContext
	app.changeDisplayContext(
		Command_Actions.view_prefix,
		Command_Contexts.schedule_byworker,
		datespan_elements
	);
};

export const requestScheduleByAssignment = function (app: AutoschedaState) {
	//const func=()=>{ //This check is done by app.changeDisplayContext
	app.changeDisplayContext(
		Command_Actions.view_prefix,
		Command_Contexts.schedule_byassignment,
		datespan_elements
	);
};

export const requestWebUsers = function (app: AutoschedaState) {
	//const func=()=>{ //This check is done by app.changeDisplayContext
	console.debug('Requesting web users.');
	app.changeDisplayContext(
		Command_Actions.view_prefix,
		Command_Contexts.web_users,
		retireable_elements
	); //Web users aren't actually deleted, they're just retired. This is because they may be associated with a worker, so their indices need to stick around in case they're referenced.
};

export const requestAssignmentTypes = function (app: AutoschedaState) {
	//const func=()=>{ //This check is done by app.changeDisplayContext
	app.changeDisplayContext(
		Command_Actions.view_prefix,
		Command_Contexts.assignment_types,
		retireable_elements
	); //Assignments aren't actually deleted, they're just retired. This is because they may have been used in the schedule.
};

export const requestScheduleTemplates = function (app: AutoschedaState) {
	//const func=()=>{ //This check is done by app.changeDisplayContext
	app.changeDisplayContext(
		Command_Actions.view_prefix,
		Command_Contexts.schedule_template_types,
		[]
	);
};

export const requestWorkerRoster = function (app: AutoschedaState) {
	//const func=()=>{ //This check is done by app.changeDisplayContext
	app.changeDisplayContext(
		Command_Actions.view_prefix,
		Command_Contexts.worker_list,
		retireable_elements
	); //Workers aren't actually deleted, they're just retired. This is because they may have been used in the schedule.
};

/*
export const requestResponsibilitySnapshot = function(app:AutoschedaState)
{
    //const func=()=>{ //This check is done by app.changeDisplayContext
    app.changeDisplayContext(Command_Actions.view_prefix, Command_Contexts.respon_snapshot, date_elements);
}
*/

export const requestResponsibility = function (app: AutoschedaState) {
	//const func=()=>{ //This check is done by app.changeDisplayContext
	app.changeDisplayContext(Command_Actions.view_prefix, Command_Contexts.responsibility, []);
};

export const requestStaging = function (app: AutoschedaState) {
	//const func=()=>{ //This check is done by app.changeDisplayContext
	app.changeDisplayContext(Command_Actions.view_prefix, Command_Contexts.staging, []);
};

export const requestGlobalRequests = function (app: AutoschedaState) {
	app.changeDisplayContext(Command_Actions.view_prefix, Command_Contexts.global_requests, []);
};

export const requestIndividualRequests = function (app: AutoschedaState) {
	app.changeDisplayContext(Command_Actions.view_prefix, Command_Contexts.individual_requests, []);
};

export const requestSolver = function (app: AutoschedaState) {
	app.changeDisplayContext(Command_Actions.view_prefix, Command_Contexts.solve, []);
};

export const requestTrackingTotals = function (app: AutoschedaState) {
	//const func=()=>{ //This check is done by app.changeDisplayContext
	app.changeDisplayContext(
		Command_Actions.view_prefix,
		Command_Contexts.tracking_totals,
		date_elements
	);
};

export const requestTrackingAdjustments = function (app: AutoschedaState) {
	//const func=()=>{ //This check is done by app.changeDisplayContext
	app.changeDisplayContext(
		Command_Actions.view_prefix,
		Command_Contexts.tracking_adjustments,
		datespan_elements
	);
};