import type { BNData } from '../../binary/BinaryNode';
import { AssignmentTypeDetails } from '../bndata/AssignmentTypeDetails';
//import { AssignmentTypeDetails } from "../bndata/AssignmentTypeDetails";
import { IDTable, IDType, type IDTypeRow } from './IDTable';

export interface AssignmentTypeRow extends IDTypeRow {
	0: number; //id
	1: string; //name
	2: BNData; //details
	3: boolean; //can't remember....
}
export class AssignmentType extends IDType<AssignmentTypeRow> {
	getID = () => {
		return this.data[0];
	};
	getName = () => {
		return this.data[1];
	};
	getDetails = () => {
		return new AssignmentTypeDetails(this.data[2]);
	};
	public static sort(a: AssignmentType, b: AssignmentType) {
		return a.getName().localeCompare(b.getName()); //alphabetical order by name!
	}
}

export class AssignmentTypeTable extends IDTable<AssignmentTypeRow, AssignmentType> {
	protected rowToType = (row: AssignmentTypeRow) => {
		return new AssignmentType(row);
	};
	public getTypesOfPriority = (priority: number) => {
		const retval = [] as AssignmentType[];
		//console.debug("Types",this.getTypes());
		this.getTypes().forEach((type) => {
			if (type.getDetails().getPriority() == priority) {
				retval.push(type);
			}
		});
		return retval;
	};
	public sortAssignmentTypeIndices = (a: number, b: number) => {
		return AssignmentType.sort(this.getType(a)!, this.getType(b)!);
	};
}
