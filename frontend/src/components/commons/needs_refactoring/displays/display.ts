import type { ASRequest, OperationContents } from '../../ancillary/ajax/commands_generic';
import { UpdateTypes } from '../../ancillary/ajax/commands_generic';
import type { AutoschedaState, MainDisplayModification } from '../../refactored/autoscheda_core';

export interface Display_Main_Props {
	request: ASRequest;
	response: OperationContents;
	app: AutoschedaState;
	display_mod: MainDisplayModification;
}

export const Display = (props: Display_Main_Props) => {
	console.debug('Starting Display render', props);
	let table = null; //<div></div>;
	switch (props.response.update_type) {
		case UpdateTypes.raw:
			//table = React.createElement(RawTable,props);
			break;
		case UpdateTypes.keyed:
			//table = React.createElement(KeyedTable,props);
			break;
		case UpdateTypes.simple:
			//table = React.createElement(SimpleTable,props);
			break;
		case UpdateTypes.staging:
			//table = React.createElement(StagingDisplay,props);
			break;
		case UpdateTypes.solve:
			//table = React.createElement(SolveDisplay,props);
			break;
		case UpdateTypes.responsibility:
			//table = React.createElement(ResponsibilityDisplay,props);
			break;
		case undefined:
			console.debug('Display type not defined.');
			break;
		default:
			console.error('Unhandled update type ' + props.response.update_type);
			console.error('Response was:');
			console.error(props.response);
			//table = (<Box>Unhandled update type {props.response.update_type}</Box>);
			break;
	}
	console.debug('Finished Display render');
	// TODO
	return table;
};
