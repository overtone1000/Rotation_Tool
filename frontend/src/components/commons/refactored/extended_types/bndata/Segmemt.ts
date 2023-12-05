import type { BNData } from '../../binary/BinaryNode';
import { OffsetDayTime } from '../../OffsetDayTime';
import { ExtendedBNData } from './ExtendedBNData';

export enum AssignmentSegmentType {
	NotWorking = 0,
	Off = 1,
	OnCall = 2,
	Task = 3,
	Working = 4,
	Moonlighting = 5
}

export class Segment extends ExtendedBNData {
	start: OffsetDayTime;
	end: OffsetDayTime;
	type: AssignmentSegmentType;

	constructor(node: BNData, extra_day_offset: number = 0) {
		super(node);

		this.type = node.c[0].v as number; //type

		let start_day_offset: number = node.c[1].c[0].c[0].v as number; //start_day_offset
		start_day_offset = extra_day_offset + start_day_offset;
		let start_time: string = node.c[1].c[0].c[1].v as string; //start_time
		this.start = new OffsetDayTime(start_day_offset, start_time);

		let end_day_offset: number = node.c[1].c[1].c[0].v as number; //end_day_offset
		end_day_offset = extra_day_offset + end_day_offset;
		let end_time: string = node.c[1].c[1].c[1].v as string; //end_timeseg.
		this.end = new OffsetDayTime(end_day_offset, end_time);
	}
}
