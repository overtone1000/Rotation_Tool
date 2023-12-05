import { addDays, differenceInDays } from 'date-fns';
import { toDateString_ShortDisplay } from '../commons/Dates';
//import { AssignmentType, AssignmentTypeTable } from "../../data_processing/extended_types/id_tables/AssignmentTypeTable";
import type {
	AssignmentType,
	AssignmentTypeTable
} from '../extended_types/id_tables/AssignmentTypeTable';
import type { RenderedAssignable } from './members/rendered_assignable';

export const dateOnly = (date: Date) => {
	return new Date(date.getFullYear(), date.getMonth(), date.getDate());
};

export const days_per_week = 7;

interface Clonable {
	clone: () => Clonable;
}

class Bucket<T extends Clonable> implements Clonable {
	public members: { [index: number]: T } = {};
	public clone() {
		//console.debug("Cloning",this);
		const retval = new Bucket<T>();
		for (const i in this.members) {
			//console.debug("Clonining member",this.members[i]);
			retval.members[i] = this.members[i].clone() as T;
		}
		return retval;
	}
}

interface Aggregates {
	priorities: Set<number>;
	type_ids: { [priority: number]: number[] }; //Leave this array so it can be sorted only once instead of every render
	priority_counts: { [i: number]: number };
	type_id_counts: { [i: number]: number };
}

class DayBucket extends Bucket<TypeBucket> implements Clonable {
	public clone() {
		const retval = new DayBucket();
		retval.members = super.clone().members;
		return retval;
	}

	public getTypeBucket(type_id: number) {
		let retval = this.members[type_id];
		if (retval === undefined) {
			retval = new TypeBucket();
			this.members[type_id] = retval;
		}
		return retval;
	}
	public addAssignable(ra: RenderedAssignable) {
		this.getTypeBucket(ra.getTypeID()).push(ra);
	}

	public getTypeCount(type: AssignmentType) {
		return this.getTypeBucket(type.getID()).length;
	}

	public getAllTypeIDs() {
		const retval = new Set<number>();
		for (const type_id in this.members) {
			retval.add(parseInt(type_id));
		}
		return retval;
	}
}

class TypeBucket extends Array<RenderedAssignable> implements Clonable {
	public clone() {
		const retval = new TypeBucket();
		for (const ra of this) {
			retval.push(ra);
		}
		return retval;
	}
}

export type RowSpan = {
	start: number;
	end: number;
};
export type WeekRenderedResult = {
	dates: Date[];
	cells: { [row: number]: { [column: number]: RenderedAssignable } };
	priority_row_spans: { [priority: number]: RowSpan };
	type_row_spans: { [type_id: number]: RowSpan };
	row_count: number;
};

export interface SummaryNode {
	worker_min: number;
	worker_max: number;
	worker_assigned: number;
	worker_active: number;
}

export class Week implements Clonable {
	protected days: Bucket<DayBucket>;
	protected start_of_week: Date;
	protected table: AssignmentTypeTable;
	protected priorities = new Set<number>();

	constructor(table: AssignmentTypeTable, start_of_week: Date) {
		this.days = new Bucket<DayBucket>();
		this.start_of_week = dateOnly(start_of_week);
		this.table = table;
	}

	public clone() {
		const retval = new Week(this.table, this.start_of_week);
		retval.priorities = new Set<number>(this.priorities);
		retval.days = this.days.clone();
		return retval;
	}

	public getSunday() {
		return this.start_of_week;
	}

	private getDayBucket(day_of_week: number) {
		let retval = this.days.members[day_of_week];
		if (retval === undefined) {
			retval = new DayBucket();
			this.days.members[day_of_week] = retval;
			//console.debug("Added daybuckut",retval);
		}
		return retval;
	}
	public addAssignable(day_of_week: number, ra: RenderedAssignable) {
		this.getDayBucket(day_of_week).addAssignable(ra);
		this.priorities.add(ra.getAssignableType()!.getDetails().getPriority());
	}
	public getAssignables(day_of_week: number, type_id: number) {
		return this.getDayBucket(day_of_week).getTypeBucket(type_id);
	}
	private getPriorityMaxCount(priority: number) {
		const types = this.table.getTypesOfPriority(priority);
		let retval = 0;
		for (const type of types) {
			let type_count = 0;
			for (const dow in this.days.members) {
				const thiscount = this.days.members[dow].getTypeCount(type);
				if (thiscount > type_count) {
					type_count = thiscount;
				}
			}
			retval += type_count;
		}
		return retval;
	}
	private getTypeMaxCount(type: AssignmentType) {
		let retval = 0;
		for (const dow in this.days.members) {
			const thiscount = this.days.members[dow].getTypeCount(type);
			if (thiscount > retval) {
				retval = thiscount;
			}
		}
		return retval;
	}
	private getAllPriorities() {
		return this.priorities;
	}
	private getTypeIDsByPriority(priorities: Set<number>) {
		const sets: { [priority: number]: Set<number> } = {};
		this.priorities.forEach((priority: number) => {
			for (const dow in this.days.members) {
				const subset = this.days.members[dow].getAllTypeIDs();
				if (subset !== undefined) {
					let this_set = sets[priority];
					if (this_set === undefined) {
						this_set = new Set<number>();
						sets[priority] = this_set;
					}

					subset.forEach((val: number) => {
						if (this.table.getType(val)!.getDetails().getPriority() == priority) {
							this_set.add(val);
						}
					});
				}
			}
		});

		//Now sort
		const retval: { [priority: number]: number[] } = {};
		for (const priority in sets) {
			retval[priority] = Array.from(sets[priority]).sort(this.table.sortAssignmentTypeIndices);
		}
		return retval;
	}
	public getAggregates() {
		const retval: Aggregates = {} as Aggregates;

		retval.priorities = this.getAllPriorities();
		retval.type_ids = this.getTypeIDsByPriority(retval.priorities);

		retval.priority_counts = {};
		retval.priorities.forEach((priority: number) => {
			retval.priority_counts[priority] = this.getPriorityMaxCount(priority);
		});

		retval.type_id_counts = {};
		for (const priority of Array.from(retval.priorities.values())) {
			retval.type_ids[priority].forEach((type_id: number) => {
				const type = this.table.getType(type_id);
				if (type) {
					retval.type_id_counts[type_id] = this.getTypeMaxCount(type);
				}
			});
		}

		return retval;
	}

	public containsDate(date: Date): boolean {
		const difference = differenceInDays(date, this.start_of_week);
		const retval = difference > 0 && difference < 7;
		//console.debug("Week contains date test",this.start_of_week.toString(),date.toString(),difference,retval);
		return retval;
	}
	public startToString() {
		return toDateString_ShortDisplay(this.start_of_week);
	}
	public endToString() {
		return toDateString_ShortDisplay(addDays(this.start_of_week, 6));
	}

	public static render_emptyweek(start_of_week: Date): WeekRenderedResult {
		const retval: WeekRenderedResult = {} as WeekRenderedResult;

		//Iterate through the days of the week
		for (let day_of_week = 0; day_of_week < days_per_week; day_of_week++) {
			const this_date = addDays(start_of_week, day_of_week);
		}

		return retval;
	}

	public render(): WeekRenderedResult {
		const aggregates = this.getAggregates();
		//console.debug("Aggregates",aggregates);

		const retval: WeekRenderedResult = {
			dates: [],
			cells: {},
			priority_row_spans: {},
			type_row_spans: {},
			row_count: 0
		};

		//Determine row spans
		const sorted_priorities = Array.from(aggregates.priorities.values()).sort(); //sort to ascending order
		{
			let current_row_base = 0;
			for (
				let priority_i = sorted_priorities.length - 1;
				priority_i >= 0;
				priority_i-- //iterate in reverse, so effectively descending priority order
			) {
				const priority = sorted_priorities[priority_i];
				const sorted_types = Array.from(aggregates.type_ids[priority].values());
				const priorty_row_start = current_row_base;
				for (const type_id of sorted_types) {
					const type_row_start = current_row_base;
					const max_for_this_type_id = aggregates.type_id_counts[type_id];
					current_row_base += max_for_this_type_id;
					const type_row_end = current_row_base - 1;
					retval.type_row_spans[type_id] = { start: type_row_start, end: type_row_end };
				}
				const priorty_row_end = current_row_base - 1;
				retval.priority_row_spans[priority_i] = { start: priorty_row_start, end: priorty_row_end };
			}
			retval.row_count = current_row_base;
		}

		for (let row = 0; row < retval.row_count; row++) {
			retval.cells[row] = {};
		}

		//Iterate through the days of the week
		for (let day_of_week = 0; day_of_week < days_per_week; day_of_week++) {
			retval.dates[day_of_week] = addDays(this.start_of_week, day_of_week);
			for (
				let priority_i = sorted_priorities.length - 1;
				priority_i >= 0;
				priority_i-- //iterate in reverse, so effectively descending priority order
			) {
				const priority = sorted_priorities[priority_i];
				const sorted_types = Array.from(aggregates.type_ids[priority].values());
				for (const type_id of sorted_types) {
					const assignables = this.getAssignables(day_of_week, type_id);
					for (let i = 0; i < assignables.length; i++) {
						const row = retval.cells[retval.type_row_spans[type_id].start + i];
						row[day_of_week] = assignables[i];
					}
				}
			}
		}

		//retval.push(<TableRow key={key}>{date_row_cells}</TableRow>);
		//console.debug("Assignable cells",assignables_cells);
		//console.debug("Row styles",rowstyles);
		/*
    for(const row_index in assignables_cells)
    {
      /*
      retval.push(
        <MemoStagingRow
          key={this.start_of_week.getDate().toString()+"_"+row_index}
          style={rowstyles[row_index]}
          cells={assignables_cells[row_index]}
        />
      );
    }
    */
		//console.debug("Retval",retval);

		return retval;
	}
}
