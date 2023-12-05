import { addDays, addWeeks, differenceInWeeks } from 'date-fns';
import type { Selectable as StagingMember } from '../../../../staging/stores';
import type { StagingOperationContents } from '../../ajax/commands_generic';
import { RenderedAssignable } from '../members/rendered_assignable';
import {
	AssignablesByIndex,
	GenericRenderedConstraint,
	type AssignablesByDate
} from '../members/rendered_constraint';
import { Week } from '../week_sorting';
import { localDateToEpochDay, type ProcessingResult01 } from './processing01';
import type { ASStagingResponseMessage, AssignableStagingData } from './stagingdata';

export interface SortedRenderedAssignables {
	rendered_assignables: AssignablesByIndex;
	rendered_assignables_by_date: AssignablesByDate;
}
export interface ProcessingResult02 extends ProcessingResult01, SortedRenderedAssignables {
	response: StagingOperationContents;
	weeks: (Week | undefined)[];
	rendered_constraints: { [i: number]: GenericRenderedConstraint };
	staging_members: { [i: number]: StagingMember };
}

//Goes through the data and calculates dates, weeks, day of week. Fills in gaps in days to make sure the grid aligns correctly with respect to the whole week.
export const processStagingData02Rendering = (preprocessing: ProcessingResult01) => {
	const t1 = new Date().getTime();

	//Process basic data
	console.debug('Processing Staging Data. Preprocessing was:', preprocessing);
	const update_data = preprocessing.response.update_data as ASStagingResponseMessage;
	const retval: ProcessingResult02 = {} as ProcessingResult02;
	retval.response = preprocessing.response;
	Object.assign(retval, preprocessing);
	retval.weeks = [];
	retval.rendered_assignables = new AssignablesByIndex();
	retval.rendered_assignables_by_date = {};
	retval.rendered_constraints = {};
	retval.staging_members = {};

	//console.debug("First sunday",first_sunday.toString());
	for (const epoch_day_index in retval.parsed_dates) {
		const pd: Date = retval.parsed_dates[epoch_day_index];
		const row_index = differenceInWeeks(pd, retval.first_sunday);

		//Javascript's Date object is based on time zones, not just dumb (but effective!) localtime like in the java backend
		//console.debug(epoch_day, date.toString());
		const day_of_week = pd.getDay(); //0 is Sunday, 1 is Monday

		if (retval.weeks[row_index] === undefined || retval.weeks[row_index] === null) {
			const start_of_week = addDays(pd, -pd.getDay());
			//console.debug("Adding week that starts on",start_of_week.toString());
			retval.weeks[row_index] = new Week(retval.assignment_types, start_of_week);
		}

		for (const assignable_key in update_data.data.assignables[epoch_day_index]) {
			const assignable_index = parseInt(assignable_key);
			const a: AssignableStagingData =
				update_data.data.assignables[epoch_day_index][assignable_index];
			const ra: RenderedAssignable = new RenderedAssignable(
				assignable_index,
				pd,
				a,
				retval.assignment_types
			);
			retval.rendered_assignables.set(assignable_index, ra);
			retval.staging_members[assignable_index] = ra;
			const epochday = localDateToEpochDay(ra.getDate());
			if (epochday) {
				if (retval.rendered_assignables_by_date[epochday] == undefined) {
					retval.rendered_assignables_by_date[epochday] = new AssignablesByIndex();
				}
				retval.rendered_assignables_by_date[epochday].set(assignable_index, ra);
			}
			retval.weeks[row_index]!.addAssignable(day_of_week, ra);
		}
	}

	for (const constraint_i in update_data.data.constraints) {
		const constraint_index = parseInt(constraint_i);
		const c_data = update_data.data.constraints[constraint_index];
		const rendered_constraint = GenericRenderedConstraint.createFromData(constraint_index, c_data)!;
		retval.rendered_constraints[constraint_index] = rendered_constraint;
		retval.staging_members[constraint_index] = rendered_constraint;
		for (const ea of Array.from(rendered_constraint.getEntailedAssignables(retval))) {
			const ra = retval.rendered_assignables.get(ea);
			if (ra) {
				ra.addConstraint(rendered_constraint);
			}
		}
	}

	if (retval.weeks.length == 0) {
		retval.weeks.push(new Week(retval.assignment_types, retval.first_sunday));
	}

	//Fill in any gaps
	let max_weeks_index = 0;
	for (let n in retval.weeks) {
		let int = parseInt(n);
		if (int > max_weeks_index) {
			max_weeks_index = int;
		}
	}
	for (let n = 0; n < max_weeks_index; n++) {
		if (retval.weeks[n] === undefined) {
			const start_of_week = addWeeks(retval.first_sunday, n);
			retval.weeks[n] = new Week(retval.assignment_types, start_of_week);
		}
	}

	const t2 = new Date().getTime();
	console.debug('Processing 02 in ' + (t2 - t1).toString() + ' ms');
	return retval;
};
