import { addDays } from 'date-fns';
import { stagingModification } from '../../../../staging/staging';
import { HighlightingStoreProvider } from '../../../../staging/table/highlighting';
import { ArrayBackedSortedSet } from '../../../needs_refactoring/commons/ArrayBackedSortedSet';
import { Staging_Directives, type ASRequestStagingParameters } from '../../ajax/commands_generic';
import type { BNData } from '../../data/BinaryNode';
import { AssignmentMember } from '../../extended_types/bndata/AssignmentMember';
import { ConstraintClass } from '../../extended_types/bndata/Constraint';
import { localDateToEpochDay } from '../data_processing/processing01';
import type { ProcessingResult02, SortedRenderedAssignables } from '../data_processing/processing02';
import {
	StagingTypes,
	type ConstraintStagingData,
	type ConstraintStagingDetails,
	type MatchOne_StagingDetails,
	type SingleWorker_StagingDetails
} from '../data_processing/stagingdata';
import type { RenderedAssignable } from './rendered_assignable';

export abstract class GenericRenderedConstraint extends HighlightingStoreProvider {
	protected data: ConstraintStagingData;
	private index: number;

	constructor(index: number, data: ConstraintStagingData) {
		super();
		this.data = data;
		this.index = index;
	}

	public getIndex() {
		return this.index;
	}

	abstract getEntailedAssignables: (preprocessing: SortedRenderedAssignables) => Set<number>;

	public getStagingType() {
		return StagingTypes.Constraint;
	}

	static createData(type: ConstraintClass, details: ConstraintStagingDetails) {
		const retval: ConstraintStagingData = {
			t: type,
			d: details
		};
		return retval;
	}

	public getConstraintClass() {
		return this.data.t;
	}
	public abstract getTokenType(
		assignable: RenderedAssignable,
		preprocessing: SortedRenderedAssignables
	): TokenType | undefined;

	public abstract validate: () => boolean;

	public abstract clearProposedDetails: (preprocessing: SortedRenderedAssignables) => void;
	public abstract hasProposedDetails: () => boolean;
	public abstract getCurrentData: () => ConstraintStagingData;

	public static createFromData(index: number, data: ConstraintStagingData) {
		console.debug('Creating constrant from data.', data.t.toString());
		switch (data.t) {
			case ConstraintClass.MatchOne:
				return new RenderedConstraint_MatchOne(index, data);
			case ConstraintClass.SingleWorker:
				return new RenderedConstraint_SingleWorker(index, data);
			default:
				console.error('Unhandled constraint class.');
				return null;
		}
	}

	public sendModification = (ras:ProcessingResult02) => {
		const parameters = {} as ASRequestStagingParameters;
		parameters.type = Staging_Directives.modify_constraint;
		parameters.staging_id = this.getIndex();
		parameters.data = this.getCurrentData();
		stagingModification(parameters);

		//console.error("Duplicate still remains with this modification?!");
		//this.clearProposedDetails(ras);
	}
}

abstract class DetailedGenericRenderedConstraint<
	T extends ConstraintStagingDetails
> extends GenericRenderedConstraint {
	private proposed_details: T | null = null;

	protected getDetails(): T {
		if (this.hasProposedDetails()) {
			return this.proposed_details!;
		} else {
			return this.getOriginalDetails();
		}
	}

	protected getOriginalDetails(): T {
		return this.data.d as T;
	}

	protected abstract cloneDetails(): T;
	protected abstract detailsAreEqual(other_details: T | null): boolean;

	private clearProposedEntailedAssignables(preprocessing: SortedRenderedAssignables) {
		for (const rai of this.getEntailedAssignables(preprocessing)) {
			const ra = preprocessing.rendered_assignables.get(rai);
			if (ra) {
				ra.removeProposedConstraint(this);
			}
		}
	}
	private updateProposedEntailedAssignables(preprocessing: SortedRenderedAssignables) {
		for (const rai of this.getEntailedAssignables(preprocessing)) {
			const ra = preprocessing.rendered_assignables.get(rai);
			if (ra) {
				ra.addProposedConstraint(this);
			}
		}
	}

	protected proposeDetails(new_details: T | null, preprocessing: SortedRenderedAssignables) {
		console.debug('Proposed details', new_details, this.getDetails());
		this.clearProposedEntailedAssignables(preprocessing);
		if (!new_details || this.detailsAreEqual(new_details)) {
			this.proposed_details = null;
		} else {
			this.proposed_details = new_details;
		}
		this.updateProposedEntailedAssignables(preprocessing);
	}

	public getCurrentData = () => {
		return {
			t: this.data.t,
			d: this.cloneDetails()
		} as ConstraintStagingData;
	};

	public clearProposedDetails = (preprocessing: SortedRenderedAssignables) => {
		this.proposeDetails(null, preprocessing);
	};
	public hasProposedDetails = () => {
		return this.proposed_details !== null;
	};
}

export class RenderedConstraint_SingleWorker extends DetailedGenericRenderedConstraint<SingleWorker_StagingDetails> {
	getAssignments = () => {
		return new ArrayBackedSortedSet<number>(this.getDetails().s);
	};

	protected cloneDetails(): SingleWorker_StagingDetails {
		const details_clone: SingleWorker_StagingDetails = {
			s: Array.from(this.getDetails().s)
		};
		return details_clone;
	}
	protected detailsAreEqual(other_details: SingleWorker_StagingDetails): boolean {
		const this_original = new RenderedConstraint_SingleWorker(this.getIndex(), this.data);
		const other_assignments = new ArrayBackedSortedSet<number>(other_details.s);
		if (!this_original.getAssignments().equals(other_assignments)) {
			return false;
		}
		return true;
	}
	public validate = () => {
		console.debug('Validating', this.getDetails());
		return (
			this.getDetails().s !== null &&
			this.getDetails().s !== undefined &&
			this.getDetails().s.length > 1
		);
	};

	public getTokenType(
		assignable: RenderedAssignable,
		preprocessing: SortedRenderedAssignables
	): TokenType | undefined {
		if (this.getEntailedAssignables().has(assignable.getIndex()!)) {
			return TokenType.SingleWorker;
		} else {
			return undefined;
		}
	}

	public proposeAssignables = (
		members: RenderedAssignable[],
		preprocessing: SortedRenderedAssignables
	) => {
		const new_details = this.cloneDetails();
		new_details.s = [];
		members.forEach((ra) => {
			const i = ra.getIndex();
			console.debug('i', i);
			if (i) {
				new_details.s.push(i);
			}
		});
		this.proposeDetails(new_details, preprocessing);
	};

	getEntailedAssignables = () => {
		return this.getAssignments().get();
	};
}

export enum MatchOneMode {
	AssignableToMatch,
	CandidateAssignables
}

export const enum TokenType {
	SingleWorker,
	MatchOne_One,
	MatchOne_Candidate
}

export class AssignablesByIndex extends Map<number, RenderedAssignable> {}
export interface AssignablesByDate {
	[date: number]: AssignablesByIndex;
}
export class RenderedConstraint_MatchOne extends DetailedGenericRenderedConstraint<MatchOne_StagingDetails> {
	constructor(index: number, data: ConstraintStagingData) {
		super(index, data);
		const details = data.d as MatchOne_StagingDetails;
	}

	getCandidateRAIndices = (preprocessing: SortedRenderedAssignables) => {
		const candidate_indices: number[] = [];
		if (preprocessing) {
			const tomatch = preprocessing.rendered_assignables.get(this.getAssignableToMatch());
			if (tomatch != undefined && tomatch != null) {
				const candidate_assignment_members = this.getCandidateAssignmentMembers();
				for (const amf of candidate_assignment_members) {
					const offset_date = addDays(tomatch.getDate(), amf.getDayOffset()!);
					const ra_this_date =
						preprocessing.rendered_assignables_by_date[localDateToEpochDay(offset_date)!];
					ra_this_date.forEach((ra, key) => {
						if (ra) {
							if (amf.getAssignmentTypeID() == ra.getAssignableType()!.getID()) {
								candidate_indices.push(key);
							}
						}
					});
				}
			}
		}
		return new ArrayBackedSortedSet<number>(candidate_indices);
	};
	getCandidateAssignmentMembers = () => {
		return AssignmentMember.fromBNData(this.getDetails().c);
	};

	public getTokenType(
		assignable: RenderedAssignable,
		preprocessing: SortedRenderedAssignables
	): TokenType | undefined {
		if (this.getAssignableToMatch() == assignable.getIndex()!) {
			return TokenType.MatchOne_One;
		} else if (this.getCandidateRAIndices(preprocessing).get().has(assignable.getIndex()!)) {
			return TokenType.MatchOne_Candidate;
		} else {
			return undefined;
		}
	}

	protected cloneDetails(): MatchOne_StagingDetails {
		const details_clone: MatchOne_StagingDetails = {
			m: this.getDetails().m,
			c: Array.from(this.getDetails().c)
		};
		return details_clone;
	}

	protected detailsAreEqual(other_details: MatchOne_StagingDetails): boolean {
		const this_original = new RenderedConstraint_MatchOne(this.getIndex(), this.data);

		//Supposed to check against the original details, not the proposed details!
		if (this_original.getDetails().m != other_details.m) {
			return false;
		}

		const this_candidate_ams = this_original.getCandidateAssignmentMembers();
		if (other_details.c.length != this_candidate_ams.length) {
			return false;
		}

		const other_candidate_ams = AssignmentMember.fromBNData(other_details.c);

		for (let n = 0; n < this_candidate_ams.length; n++) {
			const test_candidate = this_candidate_ams[n];
			let match_found = false;
			for (let m = 0; m < other_candidate_ams.length; m++) {
				const other_candidate = other_candidate_ams[m];
				if (test_candidate.equivalent(other_candidate)) {
					console.debug('SPLICE!!!');
					other_candidate_ams.splice(m, 1);
					match_found = true;
					break;
				}
			}
			if (!match_found) {
				return false;
			}
		}

		console.debug('Details are identical!', other_details, this.getDetails());
		return true;
	}

	public validate = () => {
		console.debug('Validating MatchOne', this.getDetails());
		return (
			this.getDetails().m !== null &&
			this.getDetails().m !== undefined &&
			this.getDetails().c !== null &&
			this.getDetails().c !== undefined &&
			this.getDetails().c.length > 0
		);
	};

	public proposeAssignableToMatch(
		ra: RenderedAssignable,
		preprocessing: SortedRenderedAssignables
	) {
		const new_details = this.cloneDetails();
		new_details.m = ra.getIndex();
		this.proposeDetails(new_details, preprocessing);
	}

	public proposeCandidates(candidates: BNData[], preprocessing: SortedRenderedAssignables) {
		const new_details = this.cloneDetails();
		new_details.c = candidates;
		this.proposeDetails(new_details, preprocessing);
	}

	getEntailedAssignables = (preprocessing: SortedRenderedAssignables) => {
		const retval = this.getCandidateRAIndices(preprocessing).get();
		retval.add(this.getAssignableToMatch());
		return retval;
	};

	getAssignableToMatch = () => {
		return this.getDetails().m!;
	};
}
