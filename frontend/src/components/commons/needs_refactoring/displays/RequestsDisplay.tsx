import { Box, Typography } from '@mui/material';
import { formatISO } from 'date-fns';
import React from 'react';
import { BinaryNode } from '../data_processing/binary/BinaryNode';
import { BF_Label } from '../data_processing/constants';
import {
	epochDayToLocalDate,
	shortDateString
} from '../displays/staging/data_processing/processing01';

export const createRequestDisplay = (node: BinaryNode) => {
	console.debug('Creating request display.', node);
	switch (node.node_data.l) {
		case BF_Label.SingularRequest:
			return new SingularRequest(node).toElement();
		case BF_Label.RecurringRequest:
			return new RecurringRequest(node).toElement();
		case BF_Label.SingularRequestDetails:
			return 'UNDEFINED SingularRequestDetails REQUEST DISPLAY';
		case BF_Label.RequestType:
			return 'UNDEFINED RequestType REQUEST DISPLAY';
		case BF_Label.RecurringRequestTiming:
			return new RecurringRequestTiming(node).toElement();
		case BF_Label.RecurringRequestDetails:
			return 'UNDEFINED RecurringRequestDetails REQUEST DISPLAY';
		case BF_Label.RecurringRequestChildren:
			return 'UNDEFINED ChildRequest REQUEST DISPLAY';
		case BF_Label.RecurringRequestChild:
			return new RecurringRequestDetails(node).toElement();
		default:
			console.error("This shouldn't happen.");
			return 'UNHANDLED CASE';
	}
};

const enum RequestTypes {
	Assignment,
	Group,
	If_Then
}

const enum GroupLogic {
	Exactly,
	AtLeast,
	AtMost
}

const enum RecurringRequestTimingTypes {
	Weekly,
	MonthlyByDayofWeek,
	MonthlyByDayNumber
}

abstract class RequestNode {
	protected node: BinaryNode;
	constructor(node: BinaryNode) {
		//console.debug("New node",node);
		this.node = node;
	}
	public abstract toElement: (key?: string) => React.ReactElement;
}

const box_style: React.CSSProperties = {
	marginLeft: 10
};

class TypedRequest<T extends number> extends RequestNode {
	public getTypeID = () => {
		return this.node.node_data.v as T;
	};
	public getTranslatorIndex = () => {
		return this.node.node_data.t;
	};
	public getType = () => {
		const key = this.getTypeID();
		const translator = this.node.getColumnMeta().translators[this.getTranslatorIndex()];
		if (translator !== undefined) {
			return translator[key];
		} else {
			console.error('Undefined translator.', this.node);
			return key;
		}
	};
	public toElement = (key?: string) => {
		return this.getType().toString();
	};
}

const logicToString = (type: GroupLogic) => {
	switch (type) {
		case GroupLogic.AtLeast:
			return 'At least';
		case GroupLogic.AtMost:
			return 'At most';
		case GroupLogic.Exactly:
			return 'Exactly';
	}
	return 'Invalid logic';
};

class SingularRequest extends RequestNode {
	private getTypeNode = () => {
		return new TypedRequest<RequestTypes>(this.node.children[0]);
	};
	public getDetails = () => {
		const typenode = this.getTypeNode();
		console.debug('Getting details. Type is ' + typenode.getTypeID());
		switch (typenode.getTypeID()) {
			case RequestTypes.Assignment:
				return new Assignment(this.node.children[1]);
			case RequestTypes.Group:
				return new SingularGroup(this.node.children[1]); //Skips Singular Request Details container
			case RequestTypes.If_Then:
				return new SingularIfThen(this.node.children[1]);
		}
	};

	public toElement = (key?: string) => {
		console.debug('Singular Request toElement', this);
		return this.getDetails().toElement();
	};
}

class Assignment extends RequestNode {
	public getDate = () => {
		return epochDayToLocalDate(this.node.children[0].node_data.v as number);
	};
	private getTypeNode = () => {
		return new TypedRequest<number>(this.node.children[1]);
	};
	public toElement = (key?: string) => {
		return (
			//<Box key={key} style={box_style}>
			<Typography key={key}>
				{this.getTypeNode().getType().toString() +
					' on ' +
					formatISO(this.getDate(), { representation: 'date' })}
			</Typography>
			//</Box>
		);
	};
}

abstract class Group extends RequestNode {
	public getLogicType = () => {
		return this.node.children[0].node_data.v as GroupLogic;
	};
	public getLogicNumber = () => {
		return this.node.children[1].node_data.v as number;
	};
	public abstract getChildren: () => RequestNode;
	public toElement = (key?: string) => {
		return (
			<Box key={key}>
				<Typography>
					{logicToString(this.getLogicType()) + ' ' + this.getLogicNumber() + ' of the following:'}
				</Typography>
				{this.getChildren().toElement()}
			</Box>
		);
	};
}

class SingularGroup extends Group {
	public getChildren = () => {
		return new SingularChildRequests(this.node.children[2]);
	};
}

class SingularIfThen extends RequestNode {
	private if: SingularRequest = new SingularRequest(this.node.children[0]);
	private then: SingularRequest = new SingularRequest(this.node.children[1]);
	public toElement = (key?: string) => {
		return (
			<Box key={key} style={box_style}>
				<Typography>If</Typography>
				<Box style={box_style}>{this.if.toElement()}</Box>
				<Typography>Then</Typography>
				<Box style={box_style}>{this.then.toElement()}</Box>
			</Box>
		);
	};
}

abstract class ChildRequests<T extends RequestNode> extends RequestNode {
	abstract instantiateChild: (node: BinaryNode) => T;
	public getRequests = () => {
		//console.debug("Getting children.");
		const requests = [] as T[];
		for (const childkey in this.node.children) {
			//console.debug("Child ",childkey,this.node.children[childkey]);
			requests.push(this.instantiateChild(this.node.children[childkey]));
		}
		return requests;
	};

	public toElement = (key?: string) => {
		const requests = this.getRequests();
		const arr = [] as React.ReactElement[];
		//<ListItemIcon>
		//    <CircleIcon fontSize="small" />
		//</ListItemIcon>
		let subkey = 0;
		for (const req of requests) {
			arr.push(
				<Box key={subkey++} style={{ display: 'flex', flexDirection: 'row' }}>
					<Box key="bullet" style={{ display: 'flex', alignItems: 'center' }}>
						<Typography>•</Typography>
					</Box>
					<Box key="content" style={{ flex: 'flex-grow' }}>
						{req.toElement()}
					</Box>
				</Box>
			);
		}
		return (
			<Box key={key} style={{ display: 'flex', flexDirection: 'column', ...box_style }}>
				{arr}
			</Box>
		);
	};
}

class SingularChildRequests extends ChildRequests<SingularRequest> {
	instantiateChild = (node: BinaryNode) => {
		return new SingularRequest(node);
	};
}

class RecurringChildRequests extends ChildRequests<RecurringRequestDetails> {
	instantiateChild = (node: BinaryNode) => {
		return new RecurringRequestDetails(node);
	};
}

class RelativeAssignment extends RequestNode {
	public getDayOffset = () => {
		return this.node.children[0].node_data.v as number;
	};
	private getTypeNode = () => {
		return new TypedRequest<number>(this.node.children[1]);
	};
	public toElement = (key?: string) => {
		let time_description = '';
		if (this.getDayOffset() == 0) {
			time_description = ' that day';
		} else {
			const absval = Math.abs(this.getDayOffset());
			if (absval == 1) {
				time_description = ' the day';
			} else {
				time_description = ' ' + absval.toString() + ' days';
			}

			if (this.getDayOffset() < 0) {
				time_description += ' before';
			} else {
				time_description += ' after';
			}
		}
		return (
			<Box key={key} style={box_style}>
				<Typography>{this.getTypeNode().getType() + time_description}</Typography>
			</Box>
		);
	};
}

class RecurringRequest extends RequestNode {
	public getTiming = () => {
		return new RecurringRequestTiming(this.node.children[0]);
	};
	public getDetails = () => {
		return new RecurringRequestDetails(this.node.children[1]);
	};
	public toElement = (key?: string) => {
		return (
			<Box key={key} style={box_style}>
				{this.getTiming().toElement()}
				<Box key={key} style={box_style}>
					{this.getDetails().toElement()}
				</Box>
			</Box>
		);
	};
}

class RecurringGroup extends Group {
	public getChildren = () => {
		return new RecurringChildRequests(this.node.children[2]);
	};
}

class RecurringIfThen extends RequestNode {
	private if: RecurringRequestDetails = new RecurringRequestDetails(this.node.children[0]);
	private then: RecurringRequestDetails = new RecurringRequestDetails(this.node.children[1]);
	public toElement = (key?: string) => {
		console.debug('Recurring ifthen to element', this);
		return (
			<Box key={key} style={box_style}>
				<Typography>If</Typography>
				<Box style={box_style}>{this.if.toElement()}</Box>
				<Typography>Then</Typography>
				<Box style={box_style}>{this.then.toElement()}</Box>
			</Box>
		);
	};
}

class RecurringRequestDetails extends RequestNode {
	private getTypeNode = () => {
		return new TypedRequest<RequestTypes>(this.node.children[0]);
	};

	public getDetails = () => {
		switch (this.getTypeNode().getTypeID()) {
			case RequestTypes.Assignment:
				return new RelativeAssignment(this.node.children[1]);
			case RequestTypes.Group:
				return new RecurringGroup(this.node.children[1]);
			case RequestTypes.If_Then:
				return new RecurringIfThen(this.node.children[1]);
		}
	};

	public toElement = (key?: string) => {
		const details = this.getDetails();
		let body = null;
		if (details !== undefined) {
			body = details.toElement();
		}
		return body;
	};
}

abstract class RecurrenceTiming extends RequestNode {
	private getStart = () => {
		return epochDayToLocalDate(this.node.children[0].node_data.v as number);
	};
	private getEnd = () => {
		return epochDayToLocalDate(this.node.children[1].node_data.v as number);
	};
	public dateRangeElement = () => {
		return (
			<Typography>
				{'From ' + shortDateString(this.getStart()) + ' to ' + shortDateString(this.getEnd())}
			</Typography>
		);
	};
}

class WeeklyTiming extends RecurrenceTiming {
	private getDOWs = () => {
		console.debug('Getting dows', this.node.children[2]);
		const days = [] as string[];
		for (const dow_key in this.node.children[2].children) {
			const dow: BinaryNode = this.node.children[2].children[dow_key];
			if (dow.node_data.v) {
				days.push(this.node.getColumnMeta().labels.map[dow.node_data.l]);
			}
		}

		switch (days.length) {
			case 0:
				return 'No week days selected.';
			case 1:
				return days[0];
			case 2:
				return days[0] + ' and ' + days[1];
			default:
				let retval: string = '';
				for (let n = 0; n < days.length - 1; n++) {
					retval += days[n] + ', ';
				}
				retval += ' and ' + days[days.length - 1];
				return retval;
		}
	};
	public toElement = (key?: string) => {
		return (
			<Box>
				{this.dateRangeElement()}
				<Typography>{'On every ' + this.getDOWs() + ':'}</Typography>
			</Box>
		);
	};
}

function ordinal_suffix_of(i: number) {
	const j = i % 10,
		k = i % 100;
	if (j == 1 && k != 11) {
		return i + 'st';
	}
	if (j == 2 && k != 12) {
		return i + 'nd';
	}
	if (j == 3 && k != 13) {
		return i + 'rd';
	}
	return i + 'th';
}

class MonthlyByDayofWeekTiming extends RecurrenceTiming {
	private getNumber = () => {
		return this.node.children[2].node_data.v as number;
	};
	private getDOW = () => {
		return new TypedRequest<number>(this.node.children[3]);
	};
	public toElement = (key?: string) => {
		return (
			<Box>
				{this.dateRangeElement()}
				<Typography>
					{'On the ' +
						ordinal_suffix_of(this.getNumber()) +
						' ' +
						this.getDOW().getType() +
						' of every month:'}
				</Typography>
			</Box>
		);
	};
}

class MonthlyByDayNumberTiming extends RecurrenceTiming {
	private getNumber = () => {
		return this.node.children[2].node_data.v as number;
	};
	public toElement = (key?: string) => {
		return (
			<Box>
				{this.dateRangeElement()}
				<Typography>
					{'On the ' + ordinal_suffix_of(this.getNumber()) + ' of every month:'}
				</Typography>
			</Box>
		);
	};
}

class RecurringRequestTiming extends RequestNode {
	private getTypeNode = () => {
		return new TypedRequest<RecurringRequestTimingTypes>(this.node.children[0]);
	};
	public getDetails = () => {
		switch (this.getTypeNode().getTypeID()) {
			case RecurringRequestTimingTypes.Weekly:
				//console.debug("Returning1 ",this.getTypeNode().getType());
				return new WeeklyTiming(this.node.children[1]);
			case RecurringRequestTimingTypes.MonthlyByDayofWeek:
				//console.debug("Returning2 ",this.getTypeNode().getType());
				return new MonthlyByDayofWeekTiming(this.node.children[1]);
			case RecurringRequestTimingTypes.MonthlyByDayNumber:
				//console.debug("Returning3 ",this.getTypeNode().getType());
				return new MonthlyByDayNumberTiming(this.node.children[1]);
			default:
				//console.debug("Returning4 ",this.getTypeNode().getType());
				return null;
		}
	};
	public toElement = (key?: string) => {
		const details = this.getDetails();
		let element = null;
		if (details !== undefined && details !== null) {
			element = details.toElement();
		} else {
			console.debug('Null details.', this, this.getTypeNode().getType());
		}

		return element;
	};
}
