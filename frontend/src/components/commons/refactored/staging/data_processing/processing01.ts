import { addDays, addMinutes, getUnixTime } from 'date-fns';
import type { StagingOperationContents } from '../../ajax/commands_generic';
import { AssignmentTypeTable } from '../../extended_types/id_tables/AssignmentTypeTable';
import { ScheduleTemplateTable } from '../../extended_types/id_tables/ScheduleTemplateTable';
import type { ASStagingResponseMessage } from './stagingdata';

export enum DayOfTheWeek {
	Sunday,
	Monday,
	Tuesday,
	Wednesday,
	Thursday,
	Friday,
	Saturday
}

export enum MonthsOfTheYear_full {
	January,
	February,
	March,
	April,
	May,
	June,
	July,
	August,
	September,
	October,
	November,
	December
}

export enum MonthsOfTheYear {
	Jan,
	Feb,
	Mar,
	Apr,
	May,
	Jun,
	Jul,
	Aug,
	Sep,
	Oct,
	Nov,
	Dec
}

const epoch_start = new Date(1970, 0, 0);
const seconds_per_day = 24 * 60 * 60;
const millis_per_day = seconds_per_day * 1000;

export const epochDayToLocalDate = (epoch_day: number) => {
	const millis = epoch_day * millis_per_day;
	const utc_date = new Date(millis);
	const local_date = addMinutes(utc_date, utc_date.getTimezoneOffset());
	//console.debug("Epoch day to local date:",epoch_day,millis,utc_date,local_date);
	return local_date;
};

export const localDateToEpochDay = (local_date: Date) => {
	if (local_date === undefined || local_date === null) {
		return null;
	}
	const utc_date = addMinutes(local_date, -local_date.getTimezoneOffset());
	const epoch_day = getUnixTime(utc_date) / seconds_per_day;
	return epoch_day;
};

export const shortDateString = (date: Date) => {
	return date.getMonth() + 1 + '/' + date.getDate() + '/' + date.getFullYear();
};

export interface ProcessingResult01 {
	response: StagingOperationContents;
	first_sunday: Date;
	parsed_dates: { [key: number]: Date };
	assignment_types: AssignmentTypeTable;
	schedule_template_types: ScheduleTemplateTable;
}

//Just iterates through epoch days and converts them to dates. Also calculates first sunday of each week.
export const processStagingData01Dates = (response: StagingOperationContents) => {
	const t1 = new Date().getTime();

	const retval = {} as ProcessingResult01;

	retval.response = response;

	console.debug('Processing staging data', response.update_data);
	const update_data = response.update_data as ASStagingResponseMessage;
	retval.assignment_types = new AssignmentTypeTable(update_data.assignment_types);
	retval.schedule_template_types = new ScheduleTemplateTable(update_data.schedule_template_types);

	//const row_construction:OldRow[][]=[];

	//need to parse assignables and get dates!
	retval.parsed_dates = {} as { [key: number]: Date };
	let first_date: Date | null = null;

	for (const epoch_day_index in update_data.data.assignables) {
		const epoch_day = parseInt(epoch_day_index);
		const local_date = epochDayToLocalDate(epoch_day);

		//console.debug("Parsing epoch day",epoch_day_index,date.toString());

		if (first_date === null || local_date < first_date) {
			first_date = local_date;
		}

		retval.parsed_dates[epoch_day] = local_date;
	}

	if (first_date === null) {
		first_date = new Date();
	}

	retval.first_sunday = addDays(first_date, -first_date.getDay());

	console.debug('Preprocessing finished', retval);
	const t2 = new Date().getTime();
	console.debug('Processing 01 in ' + (t2 - t1).toString() + ' ms');
	return retval;
};
