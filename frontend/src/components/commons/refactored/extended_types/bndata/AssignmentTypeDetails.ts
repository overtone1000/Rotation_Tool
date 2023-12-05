import { ExtendedBNData } from './ExtendedBNData';

export class AssignmentTypeDetails extends ExtendedBNData {
	getPriority = () => {
		return this.data.c![0].v as number;
	};
	getNodeSegments = () => {
		return this.data.c![1].c;
	};
}
