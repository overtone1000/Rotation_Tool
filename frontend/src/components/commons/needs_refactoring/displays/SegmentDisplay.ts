import { getUnixTime, max, min } from 'date-fns';
import { Debouncer } from '../commons/Debouncer';
import { OffsetDayTime } from '../data_processing/OffsetDayTime';
import type { AssignmentMember } from '../data_processing/extended_types/bndata/AssignmentMember';
import type { Segment } from '../data_processing/extended_types/bndata/Segmemt';

//const time_region_height = 50;
export const row_height = 40;
export const x_padding_ratio_right = 0.01;
export const x_padding_ratio_left = 0.01;
export const constraint_col = -1;

//export const sdwidth = 1000;

export class AssignmentDisplay {
	name: string;
	//index:number;
	segments: Segment[];
	private am: AssignmentMember;
	private isghost: boolean = false;
	constructor(name: string, am: AssignmentMember, segments: Segment[], isghost: boolean) {
		this.name = name;
		//this.index=index;
		this.segments = segments;
		this.am = am;
		this.isghost = isghost;
	}

	/*
	addSegment(segment:Segment)
	{
		this.segments.push(segment);
	}
	*/

	getAssignmentMember() {
		return this.am;
	}
	isGhost() {
		return this.isghost;
	}
}

export interface SegmentDisplayParameters {
	rows: AssignmentDisplay[][];
	segment_labels: { [i: string]: string };
}

export abstract class SegmentDisplayConstructor {
	private seg_init: SegmentDisplayParameters;
	private additional_rows: AssignmentDisplay[][] = [];

	private start_moment: Date;
	private end_moment: Date;
	right: number;
	height: number;
	left: number;
	svg_right: number;
	segments_right: number;

	constructor(init: SegmentDisplayParameters) {
		this.seg_init = init;
	}

	getAllSegments(): Segment[] {
		let segments: Segment[] = [];
		for (const row of this.getAllRows()) {
			for (const assignment of row) {
				segments = segments.concat(assignment.segments);
			}
		}
		return segments;
	}

	setAdditionalRows(rows: AssignmentDisplay[][]) {
		this.additional_rows = rows;
	}

	getInitRows() {
		return this.seg_init.rows;
	}

	getSegmentLabels() {
		return this.seg_init.segment_labels;
	}

	getAllRows() {
		const retval = this.seg_init.rows.concat(this.additional_rows);
		return retval;
	}

	momentToX(m: Date) {
		const x = getUnixTime(m);
		const x_l = getUnixTime(this.start_moment);
		const x_r = getUnixTime(this.end_moment);
		return ((x - x_l) / (x_r - x_l)) * (this.segments_right - this.left) + this.left;
	}

	timeToX(daytime: OffsetDayTime) {
		return this.momentToX(daytime.moment);
	}

	setTimeScale(timescale_as_segment: Segment) {
		this.setTimeScaleDirect(timescale_as_segment.start.moment, timescale_as_segment.end.moment);
	}

	setTimeScaleDirect(new_start_moment: Date, new_end_moment: Date) {
		this.start_moment = new_start_moment;
		this.end_moment = new_end_moment;

		console.debug('Setting time scale: ', new_start_moment, new_end_moment);
	}

	_getDays() {
		let days: number[] = [] as number[];

		const allrows = this.getAllRows();
		for (let n = 0; n < allrows.length; n++) {
			let row = allrows[n];
			for (const assignment of row) {
				for (const segment of assignment.segments) {
					let start_day = segment.start.getDayOffset();
					//if(segment.start.time=="24:00"){start_day++;}
					let end_day = segment.end.getDayOffset();
					if (segment.end.getTimeAsMinutes() <= 0) {
						end_day--;
					}
					for (let day = start_day; day <= end_day; day++) {
						days.push(day);
					}
				}
			}
		}

		return days;
	}

	_drawDayBoxes(): React.ReactNode[] {
		const retval: React.ReactNode[] = [];

		let days = this._getDays();

		let key: number = 0;
		const getKey = () => {
			return 'Daybox Element' + (key++).toString();
		};

		const allrows = this.getAllRows();

		for (const day of days) {
			const rect = new RectBounds();
			rect.top = row_height * allrows.length - 1;
			rect.bottom = rect.top + row_height;

			rect.left = this.timeToX(new OffsetDayTime(day, '00:00'));
			rect.right = this.timeToX(new OffsetDayTime(day + 1, '00:00'));

			if (rect.left < this.left) {
				rect.left = this.left;
			}
			if (rect.right > this.segments_right) {
				rect.right = this.segments_right;
			}

			retval.push(
				drawTextBox(rect, 'white', 'Day ' + day, 'black', undefined, 'black', 1, getKey())
			);
			retval.push(drawXLabel(rect.left, rect.top, rect.bottom, '', 'black', true, getKey()));
			retval.push(drawXLabel(rect.right, rect.top, rect.bottom, '', 'black', false, getKey()));

			//const midnight = "00:00";
			//let x = this.timeToX(parseInt(day), midnight);
			//time_labels[x]="Day " + day; // + " " + midnight;
		}

		return retval;
	}

	getAllMoments() {
		let moments = new Array();

		for (const row of this.getAllRows()) {
			for (const assignment of row) {
				for (const segment of assignment.segments) {
					moments.push(segment.start.moment);
					moments.push(segment.end.moment);
				}
			}
		}

		return moments;
	}

	autoScale() {
		const segments = this.getAllSegments();

		if (segments.length <= 0) {
			return;
		}

		let moments: Date[] = [];
		for (const segment of segments) {
			moments.push(segment.start.moment);
			moments.push(segment.end.moment);
		}

		this.setTimeScaleDirect(min(moments), max(moments));
	}

	abstract resize(width: number);

	public sizeToContainer(
		divref: React.MutableRefObject<{ offsetWidth: number }>,
		widthhook: WrappedHook<number>
	) {
		const handleResize = () => {
			console.debug('Resize called!', divref.current);
			if (divref.current !== null && divref.current.offsetWidth !== null) {
				widthhook.set(divref.current.offsetWidth);
			}
		};

		useEffect(() => {
			const db = new Debouncer(25, handleResize);

			db.call();

			const calldebouncedhandleresize = () => {
				widthhook.set(0);
				db.call();
			};

			window.addEventListener('resize', calldebouncedhandleresize);
			return () => {
				window.removeEventListener('resize', calldebouncedhandleresize);
			};
		}, []);
	}

	static drawSegmentBox = function (
		rect: RectBounds,
		type: number,
		concise = false,
		segment_labels: {},
		key: number = null,
		isghost: boolean,
		interaction_handler?: (evt: any) => void,
		highlight: boolean = false
	): React.ReactNode {
		let text: string;
		let textcolor: string;
		if (concise) {
			text = null;
		} else {
			text = segment_labels[type];
		}

		let color: string;
		if (isghost) {
			color = getGhostAssignmentSegmentColor(type);
			textcolor = 'white';
		} else {
			color = getAssignmentSegmentColor(type);
			textcolor = 'black';
		}

		let stroke: string = undefined;
		let strokewidth: number = undefined;
		if (highlight) {
			stroke = 'yellow';
			strokewidth = 5;
		}

		if (text === null) {
			return drawBox(rect, color, 'sb' + key, stroke, strokewidth, interaction_handler);
		} else {
			return drawTextBox(
				rect,
				color,
				text,
				textcolor,
				'middle',
				stroke,
				strokewidth,
				'sb' + key,
				interaction_handler
			);
		}
	};
}
