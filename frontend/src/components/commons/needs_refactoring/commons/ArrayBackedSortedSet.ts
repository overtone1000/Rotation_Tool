import { genericMultiselectInvert } from '../../refactored/staging/staging';

export class ArrayBackedSortedSet<T> {
	private arr: Array<T>;
	constructor(arr: Array<T>) {
		this.arr = arr;
	}
	get = () => {
		return new Set<T>(this.arr.sort());
	};
	add = (new_member: T) => {
		if (!this.arr.includes(new_member)) {
			this.arr.push(new_member);
		}
	};
	remove = (deleted_member: T) => {
		const modification_set = this.get();
		modification_set.delete(deleted_member);
		this.set(modification_set);
	};
	set = (new_set: Set<T>) => {
		this.arr.length = 0;
		new_set.forEach((val) => {
			this.arr.push(val);
		});
	};
	multiselectUpdate(multi: boolean, value: T) {
		const value_set = new Set<T>();
		value_set.add(value);
		const new_set = genericMultiselectInvert<T>(multi, value_set, this.get());
		this.set(new_set);
	}
	equals = (other: ArrayBackedSortedSet<T>) => {
		const this_set = this.get();
		const other_set = other.get();
		if (this_set.size != other_set.size) {
			return false;
		}

		const this_i = Array.from(this_set.values());
		const other_i = Array.from(other_set.values());
		for (let n = 0; n < this_i.length; n++) {
			if (this_i[n] != other_i[n]) {
				return false;
			}
		}

		return true;
	};
}
