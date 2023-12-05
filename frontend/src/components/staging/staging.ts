import {
	Command_Actions,
	Command_Contexts,
	sendPOST,
	type ASRequest,
	type ASRequestDataCommand,
	type ASRequestStagingParameters,
	type OperationContents
} from '../commons/refactored/ajax/commands_generic'; //"@migration/ts/ajax/commands_generic";
import { genericRequest } from '../commons/refactored/ajax/core';
import { stagingInit } from './stores';

export async function requestDisplayData(): Promise<OperationContents | undefined> {
	let response: OperationContents | undefined = undefined;

	const request: ASRequest = {
		action: Command_Actions.view_prefix,
		context: Command_Contexts.staging,
		parameters: {}//{ start_date: '2023-05-07', end_date: '2023-06-06', date: '2023-05-07' }
	};
	console.error('Fix dates!');

	function success_handler(raw_response: OperationContents) {
		console.debug('Success.');
		response = raw_response;
	}
	function not_allowed_handler() {
		console.error('Not authenticated for this!');
	}
	function error_handler(message: string) {
		console.error('Error!', message);
	}

	console.debug('Sending post.');
	await sendPOST(request, success_handler, not_allowed_handler, error_handler);

	console.debug('Returning', response);
	return response;
}

export function stagingModification(parameters:ASRequestStagingParameters)
{
	const onSuccess = (response:ASRequestDataCommand) => {
		console.debug("Success!",response);

		stagingInit.update(
			(init)=>{
				if(init)
				{
					if(response.updates)
					{
						for(const assignable_key in response.updates.assignables)
						{
							const intkey = parseInt(assignable_key);
							init.update_data.data.assignables[intkey]=response.updates.assignables[intkey];
						}
						for(const constraint_key in response.updates.constraints)
						{
							const intkey = parseInt(constraint_key);
							const current_constraint = init.update_data.data.constraints[intkey];
							//If a constraint existed previously, need to clean it up
							if(current_constraint)
							{
								current_constraint
							}
							init.update_data.data.constraints[intkey]=response.updates.constraints[intkey];
						}
						for(const summary_key in response.updates.summaries)
						{
							const intkey = parseInt(summary_key);
							init.update_data.data.summaries[intkey]=response.updates.summaries[intkey];
						}
					}
					if(response.deletions)
					{
						for(const assignable_key in response.updates.assignables)
						{
							const intkey = parseInt(assignable_key);
							init.update_data.data.assignables[intkey]=response.updates.assignables[intkey];
						}
						for(const constraint_key in response.updates.constraints)
						{
							const intkey = parseInt(constraint_key);
							init.update_data.data.constraints[intkey]=response.updates.constraints[intkey];
						}
					}
					if(response.messages)
					{
						for(const message of response.messages)
						{
							console.error("Handle message better",message);
						}
					}
				}
				return init;
			}
		)
	}

	genericRequest(Command_Actions.modify_prefix,Command_Contexts.staging,parameters,onSuccess);
}

/*
export async function requestDisplayData_Testing(): Promise<OperationContents | undefined> {
	const result = await fetch('/testing/staging_02.txt');
	const retval = JSON.parse(await result.text());
	return retval;
}

export async function requestDisplayData_Testing2(): Promise<OperationContents | undefined> {
	return requestDisplayData();
}
*/