import type { BNData } from '../../binary/BinaryNode';
import { ScheduleTemplateDetails } from '../bndata/ScheduleTemplateDetails';
import { IDTable, IDType, type IDTypeRow } from './IDTable';

export interface ScheduleTemplateRow extends IDTypeRow {
	0: number; //id
	1: string; //name
	2: BNData; //details
}

export class ScheduleTemplate extends IDType<ScheduleTemplateRow> {
	getID = () => {
		return this.data[0];
	};
	getName = () => {
		return this.data[1];
	};
	getDetails = () => {
		return new ScheduleTemplateDetails(this.data[2]);
	};
}

export class ScheduleTemplateTable extends IDTable<ScheduleTemplateRow, ScheduleTemplate> {
	protected rowToType = (row: ScheduleTemplateRow) => {
		return new ScheduleTemplate(row);
	};
}
