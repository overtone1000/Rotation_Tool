import { Command_Actions, sendPOST, type ASRequest, type ASRequestDataCommand, type ASRequestParameters, type CommandParameters, type Command_Contexts, type DisplayOperationContents, type GenericRow as GenericRows, type ResponseHandler } from "./commands_generic";

export const commitNew = (new_row:ASRequestParameters, current_request:ASRequest) =>
{
  console.debug("New row ",new_row);
  
  const request:ASRequest = 
  {
	action: Command_Actions.new_prefix,
	context: current_request.context,
	parameters:current_request.parameters
  };
  
  const params = request.parameters as CommandParameters;
  params.new=new_row;

  const successhandler = (e:any)=> {
    console.debug("Response:");
    console.debug(e);
  }

  sendPOST(request,successhandler,notAuthorizedHandler, errorHandler);
}

export const commitWholeRowModify = (old_response:DisplayOperationContents, new_response:DisplayOperationContents, modified_row_key:number, current_request:ASRequest) =>
{
  console.debug("Modified row " + modified_row_key);
  
  const oldrows = old_response.update_data.rows as GenericRows;
  const newrows = new_response.update_data.rows as GenericRows;
  const parameters:CommandParameters={
    old: oldrows[modified_row_key],
    new: newrows[modified_row_key]
  };

  const successhandler = (e:any)=> {
    console.debug("Response:");
    console.debug(e);
  }

  singleModify(parameters,successhandler,current_request);
}

export const genericRequest = (action:Command_Actions, context:Command_Contexts, parameters:ASRequestParameters, handler:ResponseHandler) =>
{
  const request:ASRequest = 
  {
	action: action,
	context: context,
	parameters:parameters
  };
  sendPOST(request,handler,notAuthorizedHandler, errorHandler);
}

export const singleModify = (parameters:ASRequestParameters, handler:ResponseHandler, current_request:ASRequest) =>
{
  genericRequest(Command_Actions.modify_prefix,current_request.context,parameters,handler);
}

export const bulkModify = (parameters:ASRequestDataCommand, handler:ResponseHandler, current_request:ASRequest) =>
{
  genericRequest(Command_Actions.bulk_modify_prefix,current_request.context,parameters,handler);
}

export const deleteMember = (old_response:DisplayOperationContents, new_response:DisplayOperationContents, deleted_row:number, current_request:ASRequest) =>
{
  console.debug("Delete row " + deleted_row);
  
  const parameters=current_request.parameters as CommandParameters;
  if(parameters)
  {
    const oldrows = old_response.update_data.rows as GenericRows;
    parameters.old=oldrows[deleted_row];
  }
  
  const successhandler = (e:any)=> {
    console.debug("Response:");
    console.debug(e);
  }

  genericRequest(Command_Actions.delete_prefix,current_request.context,parameters,successhandler);
}

const notAuthorizedHandler = () => {
  console.error("Not authorized.");
};

const errorHandler = (message:string) => {
  console.error("Error:",message);
};