import { AppBar, Box, CircularProgress, Grid, Toolbar, Typography } from '@mui/material';
import { ThemeProvider } from '@mui/material/styles';
import { addDays, formatISO } from 'date-fns';
import React, { FC, useMemo, useRef } from "react";
import { ASRequest, ASRequestDataCommand, ASRequestParameters, Command_Actions, Command_Contexts, Command_Parameters, DisplayOperationContents, OperationContents, ResponseHandler, checkForUnsavedWorkBeforePerformingFunction, sendPOST } from './ajax/commands_generic';
import { Message } from "./displays/staging/staging_rightpanel";
import { InfoDialog, InfoDialogProps } from "./input/dialogs";
import { WrappedHook } from "./react/WrappedHook";
import { autoscheda_theme, waiting_background_color } from "./theming/theme";

//promptNotifications();

export const default_autoscheda_styles:React.CSSProperties =
{
  backgroundColor:autoscheda_theme.palette.background.default,
  color:autoscheda_theme.palette.text.primary,
  zIndex:1
};

interface Autoscheda_Props
{

}

const createWaitingDisplay = () =>
{
  
  return (
  <Box key="Waiting Box" height="100%" width="100%" display="flex"
    style={{backgroundColor:"transparent"}}
  >
    <Grid container justifyContent="center" alignItems="center"
    style={{backgroundColor:waiting_background_color}}
    >
      <CircularProgress variant="indeterminate" color="primary" size={100} thickness={10}/>
    </Grid>
  </Box>);
}

export interface MainDisplayModification
{
  column_filters:string[]
}



export interface ControlsDisplayModification
{
  controls_minimized:boolean,
}

enum DisplayType
{
  blank,
  display_response,
  display_static
}

export class WorkTracker {
  private unsaved_work_refs:Set<WorkManager>=new Set<WorkManager>();

  public userWorkStart = (manager:WorkManager) => {
    this.unsaved_work_refs.add(manager);
    console.debug("User work start",manager);
  }

  public userWorkStop = (manager:WorkManager) => {
    this.unsaved_work_refs.delete(manager);
    console.debug("User work stop",manager);
  }

  public clear = () => {
    this.unsaved_work_refs.clear();
    console.debug("User work cleared");
  }

  public unsavedUserWork = () => {
    console.debug("User unsaved work items=" + this.unsaved_work_refs.size);
    return this.unsaved_work_refs.size>0;
  }
}

export class WorkManager {

}

export type AutoschedaDisplayParameters = {
  date?: string,
  start_date?: string,
  end_date?: string,
  show_retired?: boolean
};

export type AutoschedaDisplayParameter = keyof AutoschedaDisplayParameters;

const empty_display_modification={column_filters:null}

export class AutoschedaState {
  display_type = new WrappedHook<DisplayType>(DisplayType.blank); //AppContext.blank
  awaiting_server_response = new WrappedHook<boolean>(false);

  modal = new WrappedHook<React.ReactNode>(<div/>);
  supermodal = new WrappedHook<React.ReactNode>(<div/>);
  static_display = new WrappedHook<React.ReactNode>(<div/>);
  //current_response_display = new WrappedHook<ResponseDisplay>({});
  loading_progress = new WrappedHook<number>(50);
  building_progress = new WrappedHook<number>(50);
  
  private work_tracker = useRef<WorkTracker>(new WorkTracker());
  getWorkTracker = () => {return this.work_tracker.current;}

  current_request = new WrappedHook<ASRequest>({
    action:undefined,
    context:undefined,
    parameters:
    {
      start_date:formatISO(new Date(), { representation: 'date' }),
      end_date:formatISO(addDays(new Date(),30), { representation: 'date' }),
      date:formatISO(new Date(), { representation: 'date' }),
    }
  });

  current_response = new WrappedHook<OperationContents>({} as OperationContents);
  main_display_modification = new WrappedHook<MainDisplayModification>({column_filters:null});
  controls_display_modification = new WrappedHook<ControlsDisplayModification>({controls_minimized:true});  
  controls_elements = new WrappedHook<AutoschedaDisplayParameter[]>([]);

  constructor()
  {
    //getDisplayMeta(this.display_meta_hook);
  };

  updateDisplay(response:OperationContents)
  {
    this.main_display_modification.set(empty_display_modification);
    this.display_type.set(DisplayType.display_response);
    this.current_response.set(response);
    this.awaiting_server_response.set(false);

    console.debug("Main app state changed.");
  }
  

  setStaticDisplay(node:React.ReactNode)
  {
    this.static_display.set(node);
    this.display_type.set(DisplayType.display_static);
  }

  modifyRequestParameter = (key:string, value:any) =>
  {
    console.debug("Modifying request parameter " + key + " to " + value);
    let new_request:ASRequest={} as ASRequest;
    Object.assign(new_request,this.current_request.get());
    new_request.parameters[key] = value;
    this.sendDisplayRequest(new_request);
  };

  sendDisplayRequest(new_request:ASRequest)
  {
    console.debug("Sending display request", new_request);
    this.loading_progress.set(0);
    this.building_progress.set(0);
    //this.main_app_state.set(AppContext.waiting);
    this.awaiting_server_response.set(true);
    this.current_request.set(new_request);
    sendPOST(new_request, (response:OperationContents)=>{this.updateDisplay(response)}, this.notAuthorizedHandler, this.errorHandler);
    this.loading_progress.set(50);
  };

  commitNew = (new_row:{}) =>
  {
    console.debug("New row ",new_row);
    
    const request:ASRequest = 
    {
      action: Command_Actions.new_prefix,
      context: this.current_request.get().context,
      parameters:this.current_request.get().parameters
    };
    request.parameters[Command_Parameters.newrow]=new_row;

    const successhandler = (e:any)=> {
      console.debug("Response:");
      console.debug(e);
      this.sendDisplayRequest(this.current_request.get());
    }

    this.awaiting_server_response.set(true);
    sendPOST(request,successhandler,this.notAuthorizedHandler, this.errorHandler);
  }

  commitWholeRowModify = (old_response:DisplayOperationContents, new_response:DisplayOperationContents, modified_row_key:string) =>
  {
    console.debug("Modified row " + modified_row_key);
    
    const parameters={} as ASRequestParameters;
    parameters[Command_Parameters.oldrow]=old_response.update_data.rows[modified_row_key];
    parameters[Command_Parameters.newrow]=new_response.update_data.rows[modified_row_key];

    const successhandler = (e:any)=> {
      console.debug("Response:");
      console.debug(e);
      this.updateDisplay(new_response);
    }

    this.singleModify(parameters,successhandler);
  }

  genericRequest = (action:Command_Actions, context:Command_Contexts, parameters:ASRequestParameters, handler:ResponseHandler) =>
  {
    const request:ASRequest = 
    {
      action: action,
      context: context,
      parameters:parameters
    };
    this.awaiting_server_response.set(true);
    sendPOST(request,handler,this.notAuthorizedHandler, this.errorHandler);
  }

  singleModify = (parameters:ASRequestParameters, handler:ResponseHandler) =>
  {
    this.genericRequest(Command_Actions.modify_prefix,this.current_request.get().context,parameters,handler);
  }

  bulkModify = (parameters:ASRequestDataCommand, handler:ResponseHandler) =>
  {
    this.genericRequest(Command_Actions.bulk_modify_prefix,this.current_request.get().context,parameters,handler);
  }

  deleteMember = (old_response:DisplayOperationContents, new_response:DisplayOperationContents, deleted_row_key:string) =>
  {
    console.debug("Delete row " + deleted_row_key);
    
    const parameters=this.current_request.get().parameters;
    parameters[Command_Parameters.oldrow]=old_response.update_data.rows[deleted_row_key];
    const successhandler = (e:any)=> {
      console.debug("Response:");
      console.debug(e);
      this.updateDisplay(new_response);
    }

    this.genericRequest(Command_Actions.delete_prefix,this.current_request.get().context,parameters,successhandler);
  }

  changeDisplayContext = (action:Command_Actions, context:Command_Contexts, new_panel_elements:AutoschedaDisplayParameter[]) => {
    const func = () => {
      console.debug("Changing display context.");
      let new_request:ASRequest={} as ASRequest;
      Object.assign(new_request,this.current_request.get());
      new_request.action=action;
      new_request.context=context;

      this.controls_elements.set(new_panel_elements);
      
      this.sendDisplayRequest(new_request);
    }
    checkForUnsavedWorkBeforePerformingFunction(this,func);
  };

  logout = () => {
    console.debug("Clearing session storage...");
    sessionStorage.clear();
    console.debug(sessionStorage);
  };

  notAuthorizedHandler = () => {
    const props:InfoDialogProps = 
    {
        app:this,
        title:"Access Denied",
        message:"User not authorized.",
        confirm_button_text:"Okay",
    }
    
    const info_dialog = InfoDialog(props);
    this.display_type.set(DisplayType.blank);
    this.awaiting_server_response.set(false);
    this.showSuperDialog(info_dialog);
  };

  errorHandler = (message:string) => {
    const props:InfoDialogProps = 
    {
        app:this,
        title:"Error",
        message:message,
        confirm_button_text:"Okay",
    }
    
    const info_dialog = InfoDialog(props);
    this.awaiting_server_response.set(false);
    this.showSuperDialog(info_dialog);
  };

  messageHandler = (messages:Message[]) => {
    if(messages.length==0){return;}
    for(const message of messages)
    {
      let final_message:React.ReactElement[]=[];
      console.debug("Message Box",message);
      for(let n in message.body)
      {
        const line = message.body[n];
        final_message.push(<Typography key={n}>{line}</Typography>);
      }
      const message_element=<Box>{final_message}</Box>;
      const props:InfoDialogProps = 
      {
          app:this,
          title:message.title,
          message:message_element,
          confirm_button_text:"Okay",
      }
      
      const info_dialog = InfoDialog(props);
      //this.main_app_state.set(AppContext.blank);
      this.showSuperDialog(info_dialog);
    }    
  }

  showDialog = (new_dialog:React.ReactNode|null) => {
    this.modal.set(new_dialog);
  };

  showSuperDialog = (new_dialog:React.ReactNode|null)=> {
    this.supermodal.set(new_dialog);
  }
}

export const AutoschedaFC:FC<Autoscheda_Props> = () => {
  const app_state = new AutoschedaState();
  
  console.debug("Autoscheda Render Start. DisplayType " + app_state.display_type.get().toString()); 

  const response_display = MemoizedResponseDisplay(app_state);
   
  //Handle display type
  let main_display:React.ReactNode=null;
  let control_panel:React.ReactNode=null;
  switch(app_state.display_type.get())
  {
    case DisplayType.display_response:
      main_display=response_display.main;
      control_panel=response_display.controls;
      break;
    case DisplayType.display_static:
      main_display=app_state.static_display.get();
      break;
    case DisplayType.blank:
      //Everything just stays null
      break;
  }

  //Handle waiting
  let waiting_display:React.ReactNode=null;
  if(app_state.awaiting_server_response.get())
  {
    console.debug("Loading progress = " + app_state.loading_progress.get());
    console.debug("Building progress = " + app_state.building_progress.get());
    waiting_display=(
      <Box position="absolute" top="0px" left="0px" style={{zIndex:2, backgroundColor:"transparent"}} width="100%" height="100%">
        {createWaitingDisplay()}
      </Box>
    );
  }

  //{snackbar()} //No notifications for now

  const retval = (
    <ThemeProvider theme={autoscheda_theme}>
      <Box display="flex" flexDirection="column" width="100%" height="100vh" overflow="hidden">      
        <AppBar position="static" color="inherit" style={{width:"100%", height:"min-content"}}>
          <Toolbar>
            {createAutoschedaMenu(app_state)}              
          </Toolbar>
        </AppBar>
        <Box position="relative" width="100%" height="100%">
          {waiting_display}
          <Box position="absolute" top="0px" left="0px" style={default_autoscheda_styles} minWidth="0px" width="100%" height="100%" display="flex" flexDirection="row" >
            <Box minWidth="0px" key="Main Display Box" flex="1 1 0px">
              {main_display}
            </Box>
            <Box key="Controls Box" flex="0 0 content" overflow="scroll">
              {control_panel}
            </Box>
          </Box>
          {app_state.modal.get()}
          {app_state.supermodal.get()}
        </Box>
      </Box>
    </ThemeProvider>
  );

  console.debug("Autoscheda Render Finish");
  return retval;
}

const MemoizedResponseDisplay = (state:AutoschedaState) => {

  const common_dependencies:any[] = [
    state.current_response.get(),
    //state.current_request.get(), //Is this really necessary? Causes lots of rerendering    
  ];

  const main_dependencies = Array.from(common_dependencies);
  main_dependencies.push(state.main_display_modification.get());

  const control_panel_dependencies = Array.from(common_dependencies);
  control_panel_dependencies.push(state.controls_display_modification.get());
  //DON'T make state.controls_elements a dependency!

  //Rendering display and controls are expensive. These need to be memoized like so. This prevents rerendering of the components if any of their dependencies is unchanged.
  const request_display = useMemo(() => {
    console.debug("Request display memo function update: main",state,main_dependencies);
    return (<Display
    app={state}
    response={state.current_response.get()}
    request={state.current_request.get()}
    display_mod={state.main_display_modification.get()
    }
  />)},main_dependencies);

  const request_control_panel = useMemo(() => {
    console.debug("Request display memo function update: control panel",state,control_panel_dependencies);
    return (<Controls
    app={state}
    response={state.current_response.get()}
    request={state.current_request.get()}
    display_mod={state.controls_display_modification.get()}
    display_parameters={state.controls_elements.get()}
  />)},control_panel_dependencies); 

  return {
    main:request_display,
    controls:request_control_panel
  }
}