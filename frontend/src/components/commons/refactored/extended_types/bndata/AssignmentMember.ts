import type { BNData } from '../../data/BinaryNode';
import type { RenderedAssignable } from '../../staging/members/rendered_assignable';
import { ExtendedBNData } from './ExtendedBNData';

export class AssignmentMember extends ExtendedBNData {
	constructor(data: BNData) {
		super(data);
	}
	getAssignmentTypeID = () => {
		if (this.data.c) {
			return this.data.c[0].v as number;
		}
	};
	getDayOffset = () => {
		if (this.data.c) {
			return parseInt(this.data.c[1].v as string);
		}
	};
	equivalent = (am: AssignmentMember) => {
		return (
			this.getAssignmentTypeID() == am.getAssignmentTypeID() &&
			this.getDayOffset() == am.getDayOffset()
		);
	};

	public static fromBNData(data: BNData[]) {
		return data.map((value: BNData) => {
			return new AssignmentMember(value);
		});
	}
}

export class IndexedAssignmentMember extends AssignmentMember {
	private index: number;
	constructor(index: number, data: BNData) {
		super(data);
		this.index = index;
	}
	getIndex = () => {
		return this.index;
	};
}

export class LinkedAssignmentMember extends AssignmentMember {
	private ra: RenderedAssignable;
	constructor(ra: RenderedAssignable, data: BNData) {
		super(data);
		this.ra = ra;
	}
	getRenderedAssignable = () => {
		return this.ra;
	};
}
