import type { Column } from '../../ajax/commands_generic';
import type { DataMeta } from '../../data_types';
import type { ExtendedBNData } from '../bndata/ExtendedBNData';

export interface IDTypeRow {}

export interface IDTypeTableData<T extends IDTypeRow> {
	cols: { [i: number]: Column };
	rows: { [i: number]: T };
	meta: DataMeta;
}

export abstract class IDType<T extends IDTypeRow> {
	public data: T | null = null;
	constructor(data: T) {
		this.data = data;
	}
	abstract getID: () => number;
	abstract getName: () => string;
	abstract getDetails: () => ExtendedBNData;
}

export abstract class IDTable<T extends {}, U extends IDType<T>> {
	protected data: IDTypeTableData<T>;
	protected instances: Map<number, U> | null = null;
	protected order: number[] = [];
	protected built = false;
	constructor(data: IDTypeTableData<T>) {
		this.data = data;
	}
	private checkBuild = () => {
		if (this.built) {
			return;
		}
		this.instances = new Map<number, U>();
		for (const row_index in this.data.rows) {
			const row = this.data.rows[row_index];
			const type = this.rowToType(row);
			this.instances.set(type.getID(), type);
			this.order.push(type.getID()); //Preserve row order so lists created with the getLabels function are in the order provided by the table.
		}
		this.built = true;
	};
	protected abstract rowToType: (row: T) => U;
	public getType = (id: number) => {
		this.checkBuild();
		return this.instances!.get(id);
	};
	public getTypes = () => {
		this.checkBuild();
		return this.instances;
	};
	public getRowIndices = () => {
		this.checkBuild();
		return Object.keys(this.data.rows);
	};
	public getIDs = () => {
		this.checkBuild();
		return this.order;
	};
	public getLabels = () => {
		this.checkBuild();
		const retval: { [index: string]: string } = {};
		this.order.forEach((value) => {
			retval[value] = this.instances!.get(value)!.getName();
			console.debug('For each', value, retval[value], retval);
		});
		return retval;
	};
}
