import { addDays, addHours, addMinutes, format, intervalToDuration, parse } from 'date-fns';

//const time_format = "YYYYMMDDhh:mm";

const time_only_format = 'HH:mm';

const reference_date = new Date(2001, 0, 1);

export interface OffsetDayTimeInterval {
	start: OffsetDayTime;
	end: OffsetDayTime;
}

export class OffsetDayTime {
	moment: Date;
	constructor(day_offset?: number, time?: string) {
		if (time) {
			this.moment = parse(time, time_only_format, reference_date);
		} else {
			this.moment = reference_date;
		}

		if (day_offset) {
			this.moment = addDays(this.moment, day_offset);
		}
	}
	static toDate(day_offset: number = 0, hours: number = 0, minutes: number = 0) {
		//return moment(date_constant+time, time_format).add(day_offset,"d");
		let retval = addDays(reference_date, day_offset);
		retval = addHours(retval, hours);
		retval = addMinutes(retval, minutes);
		return retval;
	}
	getTime(): string {
		return format(this.moment, time_only_format);
	}
	getDayOffset(): number {
		let int: Interval;
		const is_positive = this.moment > reference_date;
		if (is_positive) {
			//offset is positive
			int = {
				start: reference_date,
				end: this.moment
			};
		} else {
			int = {
				start: this.moment,
				end: reference_date
			};
		}
		const dur: Duration = intervalToDuration(int);
		let retval = 0;
		if (dur.days) {
			retval = Math.floor(dur.days);
			if (!is_positive) {
				retval *= -1;
			}
		}
		return retval;
	}
	getTimeAsMinutes(): number {
		//return differenceInMinutes(reference_date,this.moment);
		return this.moment.getHours() * 60 + this.moment.getMinutes();
	}
}
