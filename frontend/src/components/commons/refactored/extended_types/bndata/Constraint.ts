import type { BNData } from '../../data/BinaryNode';
import { AssignmentMember, IndexedAssignmentMember } from './AssignmentMember';
import { ExtendedBNData } from './ExtendedBNData';

export class Constraint extends ExtendedBNData {
	details!: ConstraintDetails;

	constructor(data: BNData) {
		super(data);

		const children = data.c;
		if (children) {
			switch (this.getType()) {
				case ConstraintClass.MatchOne: //Match 1
					this.details = new MatchOne(children[1] as BNData);
					break;
				case ConstraintClass.SingleWorker: //Single Worker
					this.details = new SingleWorker(children[1] as BNData);
					break;
			}
		}
	}
	getType(): ConstraintClass | null {
		return this.data.c![0].v as ConstraintClass;
	}
}

export abstract class ConstraintDetails {
	data: BNData;
	constructor(data: BNData) {
		this.data = data;
	}
	abstract getInvolvedAssignments(assignments: AssignmentMember[]): number[];
	abstract handleAssignmentDeletion(index: number): boolean;
	abstract handleAssignmentIndexSwap(index_1: number, index_2: number): void;
}

const indexSwapResult = (candidate: number, index_1: number, index_2: number) => {
	if (candidate == index_1) {
		return index_2;
	} else if (candidate == index_2) {
		return index_1;
	} else {
		return candidate;
	}
};

export class MatchOne extends ConstraintDetails {
	constructor(data: BNData) {
		super(data);
	}

	getToMatchAssignmentMemberIndex() {
		return this.data.c![0].v as number;
	}

	private candidates!: number[];
	private uams!: IndexedAssignmentMember[];

	build(assignments: AssignmentMember[]) {
		this.candidates = [];
		this.uams = [];
		const to_match = assignments[this.getToMatchAssignmentMemberIndex()];
		let next_ghost_index = assignments.length;
		for (const amfd of this.data.c![1].c!) {
			const amf = new AssignmentMember(amfd);
			let match_found = false;
			for (const ass_i in assignments) {
				const ass = assignments[ass_i];
				if (ass.equivalent(amf)) {
					match_found = true;
					this.candidates.push(parseInt(ass_i));
					break;
				}
			}
			if (!match_found) {
				this.uams.push(new IndexedAssignmentMember(next_ghost_index++, amfd));
			}
		}
	}
	getCandidates() {
		return [...this.candidates];
	}
	getUnavailableAssignmentMembers() {
		return [...this.uams];
	}

	getCandidatesAndUAMs() {
		const retval: number[] = this.getCandidates();
		for (const uam of this.uams) {
			retval.push(uam.getIndex());
		}
		return retval;
	}

	getInvolvedAssignments() {
		const retval = this.getCandidatesAndUAMs();
		retval.push(this.getToMatchAssignmentMemberIndex());
		return retval;
	}

	handleAssignmentDeletion(index: number) {
		if (this.getToMatchAssignmentMemberIndex() == index) {
			return true; //just delete the match one if its assignment is deleted
		} else if (this.getToMatchAssignmentMemberIndex() > index) {
			(this.data.c![0].v as number)--;
		}
		/*
        removeIndexFromList(this.data.c[1], index);
        if(this.getCandidates().length<=0)
        {
            return true;
        }
        */
		return false;
	}

	handleAssignmentIndexSwap(index_1: number, index_2: number) {
		this.data.c![0].v = indexSwapResult(this.data.c![0].v as number, index_1, index_2);
		/*
        const candidates=this.data.c[1].c;
        for(let n=0;n<candidates.length;n++)
        {
            candidates[n].v=indexSwapResult(candidates[n].v as number,index_1,index_2);
        }
        */
	}
}

export class SingleWorker extends ConstraintDetails {
	constructor(data: BNData) {
		super(data);
	}
	getAssignments() {
		return getIndexList(this.data.c![0]);
	}
	getInvolvedAssignments(assignments: AssignmentMember[]) {
		return this.getAssignments();
	}

	handleAssignmentDeletion(index: number) {
		removeIndexFromList(this.data.c![0], index);
		if (this.getAssignments().length <= 0) {
			return true;
		}
		return false;
	}

	handleAssignmentIndexSwap(index_1: number, index_2: number) {
		const members = this.data.c![0].c;
		if (members) {
			for (let n = 0; n < members.length; n++) {
				members[n].v = indexSwapResult(members[n].v as number, index_1, index_2);
			}
		}
	}
}

const getIndexList = function (node: BNData) {
	//console.debug("Getting index list.",node);
	let retval = new Array();
	for (const member of node.c!) {
		retval.push(member.v);
	}
	return retval;
};

const removeIndexFromList = function (node: BNData, node_index_to_remove: number) {
	for (
		let n = node.c!.length - 1;
		n >= 0;
		n-- //Go backwards for safe splicing
	) {
		const val = node.c![n].v as number;
		if (val == node_index_to_remove) {
			node.c!.splice(n, 1);
		} else if (val > node_index_to_remove) {
			(node.c![n].v as number)--;
		}
	}
	//console.debug("Final index list ",node.c);
};

export enum ConstraintClass {
	MatchOne = 0,
	SingleWorker = 1
}
