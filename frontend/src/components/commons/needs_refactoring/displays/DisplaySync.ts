import { isAfter, isBefore } from 'date-fns';
import type { OffsetDayTime, OffsetDayTimeInterval } from '../data_processing/OffsetDayTime';
import type { Segment } from '../data_processing/extended_types/bndata/Segmemt';

type Listener = (interval: OffsetDayTimeInterval) => void;

export class DisplaySync {
	interval: OffsetDayTimeInterval = {} as OffsetDayTimeInterval;
	listeners: Listener[] = [] as Listener[];

	constructor() {}

	addDayTime(newodt: OffsetDayTime) {
		let update = false;
		if (this.interval.start === undefined || this.interval.end === undefined) {
			this.interval = {
				start: newodt,
				end: newodt
			};
			update = true;
		} else {
			if (isBefore(newodt.moment, this.interval.start.moment)) {
				this.interval.start = newodt;
				update = true;
			}
			if (isAfter(newodt.moment, this.interval.end.moment)) {
				this.interval.end = newodt;
				update = true;
			}
		}

		if (update) {
			this.sync();
		}
	}

	addSegment(seg: Segment) {
		this.addDayTime(seg.start);
		this.addDayTime(seg.end);
	}

	addListener(new_listener: Listener) {
		console.debug('Adding listener');
		this.listeners.push(new_listener);
	}

	sync() {
		console.debug(
			'Syncer is updating ' +
				this.listeners.length +
				' listeners to ' +
				this.interval.start.moment.toString() +
				' to ' +
				this.interval.end.moment.toString()
		);
		const new_int = { start: this.interval.start, end: this.interval.end }; //send as new object to make hooks go!
		for (const listener of this.listeners) {
			listener(new_int);
		}
	}
}
