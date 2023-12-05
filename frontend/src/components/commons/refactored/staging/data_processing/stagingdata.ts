import { deepBNDataClone, type BNData } from '../../data/BinaryNode';
import { DataType, type DataMeta } from '../../data_types';
import type { ConstraintClass } from '../../extended_types/bndata/Constraint';
import type { AssignmentTypeRow } from '../../extended_types/id_tables/AssignmentTypeTable';
import type { IDTypeTableData } from '../../extended_types/id_tables/IDTable';
import type { ScheduleTemplateRow } from '../../extended_types/id_tables/ScheduleTemplateTable';
import type { SummaryNode } from '../week_sorting';
//import { AssignmentTypeRow } from "../extended_types/id_tables/AssignmentTypeTable";

export enum StagingTypes {
	Assignable = 0,
	Constraint = 1
}

export interface AssignableStagingData {
	t: number; //type id
	w: number; //worker id
	l: boolean; //locked
	c: [number]; //candidates for this assignment
}

export interface ConstraintStagingData {
	t: ConstraintClass; //constraint type id
	d: ConstraintStagingDetails; //constraint details
}

export type ConstraintStagingDetails = MatchOne_StagingDetails | SingleWorker_StagingDetails;

//public abstract cloneDetails:()=>ConstraintStagingData;
export interface MatchOne_StagingDetails {
	m: number | undefined; //matched assignment index
	c: BNData[];
}

export function MatchOneStagingDetailsCandidatesToBNDataClone(
	details: MatchOne_StagingDetails
): BNData {
	const as_bn_data = {
		c: details.c,
		l: 16, //from BinaryMetaProvider.java
		d: DataType.Binary,
		i: instantiator_keys.match_one_candidates,
		r: false,
		h: false
	};

	return deepBNDataClone(as_bn_data);
}

export interface SingleWorker_StagingDetails {
	s: number[]; //assignment indices to be worked by a single worker
}

/*
export const StagingDetailsAreEqual = (a:ConstraintStagingDetails,b:ConstraintStagingDetails) =>
{
  return JSON.stringify(a)===JSON.stringify(b);
}
*/

export interface ASStagingResponseData {
	assignables: {
		[epoch_day: number]: {
			[i: number]: AssignableStagingData;
		};
	};
	constraints: { [i: number]: ConstraintStagingData };
	summaries: { [i: number]: SummaryNode };
}

const instantiator_keys = {
	match_one_candidates: 'match_one_candidates'
};

export interface ASStagingResponseMessage {
	data: ASStagingResponseData;
	meta: DataMeta;
	colmeta: DataMeta;
	schedule_template_types: IDTypeTableData<ScheduleTemplateRow>;
	assignment_types: IDTypeTableData<AssignmentTypeRow>;
	instantiators: {
		[keys: string]: BNData;
	};
	commitable: number[];
}
