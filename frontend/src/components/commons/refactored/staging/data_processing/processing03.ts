import { addDays, addWeeks, differenceInWeeks, isBefore } from 'date-fns';
import { ScheduleTemplateTable } from '../../extended_types/id_tables/ScheduleTemplateTable';
import { RenderedAssignable } from '../members/rendered_assignable';
import type { ProposedAddition } from '../staging';
import { Week } from '../week_sorting';
import type { ProcessingResult02 } from './processing02';
import type { ASStagingResponseMessage, AssignableStagingData } from './stagingdata';

const partialCloneProcessingResult02 = (original: ProcessingResult02) => {
	const retval: ProcessingResult02 = {} as ProcessingResult02;
	Object.assign(retval, original);

	//Copy weeks
	retval.weeks = [];
	for (const i in original.weeks) {
		retval.weeks[i] = original.weeks[i]!.clone();
	}

	return retval;
};

export enum AddType {
	Assignment,
	ScheduleTemplate,
	Constraint
}
export function addTypeToString(type: AddType) {
	switch (type) {
		case AddType.Assignment:
			return 'Assignment';
		case AddType.ScheduleTemplate:
			return 'Schedule Template';
		case AddType.Constraint:
			return 'Constraint';
		default:
			console.error('Unhandled type', type);
			return 'Unhandled type.';
	}
}

export const processStagingData03ProposedChanges = (
	preprocessing: ProcessingResult02,
	selected_date: Date | undefined,
	proposed_addition: ProposedAddition | undefined
) => {
	const t1 = new Date().getTime();

	//Now add the proposed changes
	if (
		proposed_addition !== null &&
		proposed_addition !== undefined &&
		proposed_addition.context !== null &&
		proposed_addition.context !== undefined &&
		proposed_addition.selected_type !== null &&
		proposed_addition.selected_type !== undefined &&
		selected_date !== null &&
		selected_date !== undefined
	) {
		//&& proposed_addition.selected_date!==null && proposed_addition.selected_date!==undefined)
		const retval = partialCloneProcessingResult02(preprocessing); //Clones weeks so the following changes don't persist between calls to this function

		const proposed_index_prefix = 'proposed';

		const addAssignable = (type: number, date: Date, index?: string) => {
			let proposed_week_index = differenceInWeeks(date, retval.first_sunday);
			if (isBefore(date, retval.first_sunday)) {
				proposed_week_index -= 1;
			} //differenceInWeeks rounds towards 0
			if (proposed_week_index < 0) {
				retval.first_sunday = addWeeks(retval.first_sunday, proposed_week_index);
				const front = [] as (Week | undefined)[];
				for (let n = 0; n < -proposed_week_index; n++) {
					front.push(undefined);
				}
				retval.weeks = front.concat(retval.weeks);
				proposed_week_index = 0;
				console.debug('Added weeks to the beginning. Now:', retval);
			}

			console.debug('Adding proposed assignable ', type, date, index, proposed_week_index);
			const proposed_dow = date.getDay();
			const adata: AssignableStagingData = {
				t: type,
				w: -1,
				c: [-1],
				l: false
			};
			let proposed_index = 'proposed_index_prefix';
			if (index !== undefined) {
				proposed_index += ' ' + index.toString();
			}
			const ra = new RenderedAssignable(undefined, date, adata, retval.assignment_types);
			retval.rendered_assignables[proposed_index] = ra;

			if (retval.weeks[proposed_week_index] === undefined) {
				const new_week_sunday: Date = addWeeks(retval.first_sunday, proposed_week_index);
				retval.weeks[proposed_week_index] = new Week(
					preprocessing.assignment_types,
					new_week_sunday
				);
			}
			retval.weeks[proposed_week_index]!.addAssignable(proposed_dow, ra);
		};

		const addScheduleTemplate = (type: number, date: Date) => {
			const update_data = retval.response.update_data as ASStagingResponseMessage;
			const sg_type_table: ScheduleTemplateTable = new ScheduleTemplateTable(
				update_data.schedule_template_types
			);
			const sg_type = sg_type_table.getType(type);
			if (sg_type !== undefined) {
				const assignment_members = sg_type.getDetails().getAssignmentMembers();
				for (const sm_index in assignment_members) {
					const sm = assignment_members[sm_index];
					addAssignable(
						sm.getAssignmentTypeID()!,
						addDays(date, sm.getDayOffset()!),
						proposed_index_prefix + ' ' + sm_index
					);
					console.debug('Add the assignable!');
				}
				const constraint_members = sg_type.getDetails().getConstraintMembers();
				for (const cm_index in constraint_members) {
					const c = constraint_members[cm_index];
					console.error('Not rendered! Need to implement!');
					/*
          let rc:GenericRenderedConstraint;
          const fake_index = Object.keys(retval.rendered_constraints).length;
          switch(c.getType())
          {
            case ConstraintClass.SingleWorker:
              rc = new RenderedConstraint_SingleWorker(fake_index, c);
              break;
            case ConstraintClass.MatchOne:
              rc = new RenderedConstraint_MatchOne();
              break;
          }
          */
					//retval.rendered_constraints[Object.keys(retval.rendered_constraints).length]=c;
				}
			}
		};

		const addConstraint = () => {
			retval.rendered_constraints[proposed_addition.constraint!.getIndex()] !=
				proposed_addition.constraint;
		};

		switch (proposed_addition.context) {
			case AddType.Assignment:
				console.debug('Adding proposed assignment', proposed_addition.selected_type, selected_date);
				for (let n = 0; n < proposed_addition.multiple; n++) {
					addAssignable(proposed_addition.selected_type, selected_date);
				}
				break;
			case AddType.ScheduleTemplate:
				console.debug(
					'Adding proposed schedule template',
					proposed_addition.selected_type,
					selected_date
				);
				for (let n = 0; n < proposed_addition.multiple; n++) {
					addScheduleTemplate(proposed_addition.selected_type, selected_date);
				}
				break;
			case AddType.Constraint:
				console.debug('Adding proposed constraint', proposed_addition.constraint);
				addConstraint();
				break;
		}

		const t2 = new Date().getTime();
		console.debug('Processing 03 in ' + (t2 - t1).toString() + ' ms');
		return retval;
	} else {
		console.debug('Aborting processing 3', selected_date, proposed_addition);
		return preprocessing;
	}
};
