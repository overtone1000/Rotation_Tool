import { derived, writable, type Updater } from 'svelte/store';
import { HighlightingStoreProvider } from '../../../../staging/table/highlighting';
import type { AssignmentTypeTable } from '../../extended_types/id_tables/AssignmentTypeTable';
import { StagingTypes, type AssignableStagingData } from '../data_processing/stagingdata';
import type { GenericRenderedConstraint } from './rendered_constraint';

export class RenderedAssignable extends HighlightingStoreProvider {
	private index: number | undefined;
	private date: Date;
	private assignable: AssignableStagingData;

	private constraints = writable<Map<number,GenericRenderedConstraint>>(
		new Map<number,GenericRenderedConstraint>()
	);

	private proposed_constraints = writable<Map<number,GenericRenderedConstraint>>(
		new Map<number,GenericRenderedConstraint>()
	);

	private aggregate_constraints = derived(
		[this.constraints,this.proposed_constraints],
		([base,proposed])=>{
			const retval = new Map<number,GenericRenderedConstraint>();
			base.forEach((value,key)=>{retval.set(key,value);});
			proposed.forEach((value,key)=>{retval.set(key,value);});
			return retval;
		}
	);

	private table: AssignmentTypeTable;

	constructor(
		index: number | undefined,
		date: Date,
		a: AssignableStagingData,
		table: AssignmentTypeTable
	) {
		super();
		this.index = index;
		this.assignable = a;
		this.table = table;
		this.date = date;
	}

	public getIndex = () => {
		return this.index;
	};
	public getAssignable = () => {
		return this.assignable;
	};
	public getTypeID = () => {
		return this.assignable.t;
	};
	public isLocked = () => {
		return this.assignable.l;
	};
	public getAssignableType = () => {
		return this.table.getType(this.getTypeID());
	};
	public getAssignedWorker = () => {
		return this.assignable.w;
	};
	public getCandidates = () => {
		return this.assignable.c;
	};

	public getStagingType() {
		return StagingTypes.Assignable;
	}

	public getDate = () => {
		return this.date;
	};

	private static addUpdate(c: GenericRenderedConstraint): Updater<Map<number,GenericRenderedConstraint>> {
		return (current: Map<number,GenericRenderedConstraint>) => {
			current.set(c.getIndex(),c);
			return current;
		};
	}

	private static removeUpdate(
		c: GenericRenderedConstraint
	): Updater<Map<number,GenericRenderedConstraint>> {
		return (current: Map<number,GenericRenderedConstraint>) => {
			current.delete(c.getIndex());
			return current;
		};
	}

	private static clearUpdate(): Updater<Map<number,GenericRenderedConstraint>> {
		return (current: Map<number,GenericRenderedConstraint>) => {
			return new Map<number,GenericRenderedConstraint>();
		};
	}

	public addConstraint = (c: GenericRenderedConstraint) => {
		console.debug("Adding constraint",c);
		this.constraints.update(RenderedAssignable.addUpdate(c));
	};

	public addProposedConstraint = (c: GenericRenderedConstraint) => {
		console.debug("Adding proposed constraint",c);
		this.proposed_constraints.update(RenderedAssignable.addUpdate(c));
	};

	public removeProposedConstraint = (c: GenericRenderedConstraint) => {
		this.proposed_constraints.update(RenderedAssignable.removeUpdate(c));
	};

	public clearProposedConstraints = () => {
		this.proposed_constraints.update(RenderedAssignable.clearUpdate());
	};

	public getConstraintsStore = () => {
		return this.aggregate_constraints;
	};

	public static compare = (a: RenderedAssignable, b: RenderedAssignable) => {
		const date_comp = a.getDate().valueOf() - b.getDate().valueOf(); //Lower first!
		if (date_comp !== 0) {
			return date_comp;
		}
		const atype = a.getAssignableType();
		const btype = b.getAssignableType();
		if (atype && btype) {
			const level_comp = atype.getDetails().getPriority() - btype.getDetails().getPriority(); //Higher first!
			if (level_comp !== 0) {
				return level_comp;
			}
			const name_comp = atype.getName().localeCompare(btype.getName()); //Alphabetical!
			if (name_comp !== 0) {
				return name_comp;
			}
		}
		return 1;
	};
}
