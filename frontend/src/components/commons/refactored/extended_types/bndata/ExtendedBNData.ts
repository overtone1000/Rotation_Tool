import type { BNData } from '../../binary/BinaryNode';

export abstract class ExtendedBNData {
	protected data: BNData;
	constructor(data: BNData) {
		this.data = data;
	}
}
