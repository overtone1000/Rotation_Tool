import { drawTextBox, drawXLabel } from '../svg/functions';
import { RectBounds } from '../svg/RectBounds';

import { BNData } from '../../refactored/data/BinaryNode';
import { DataMeta } from '../data_processing/data_types';
import { Segment } from '../data_processing/extended_types/bndata/Segmemt';
import { OffsetDayTimeInterval } from '../data_processing/OffsetDayTime';
import { DisplaySync } from './DisplaySync';
import {
	AssignmentDisplay,
	row_height,
	SegmentDisplayConstructor,
	SegmentDisplayParameters,
	x_padding_ratio_left,
	x_padding_ratio_right
} from './SegmentDisplay';

export const addSegmentsToSyncer = function (syncer: DisplaySync, members: BNData[]) {
	//console.debug("Adding segments to syncer.");
	for (const node_segment of members) {
		const segment = new Segment(node_segment);
		//console.debug(segment);
		syncer.addSegment(segment);
	}
};

export interface AssignmentSegmentDisplayProps {
	key: string;
	members: BNData[];
	column_meta: DataMeta;
	syncer: DisplaySync | undefined;
	individual_rows: boolean;
	segment_select_hook: undefined | WrappedHook<string | null>;
}

export const AssignmentSegmentDisplay: FC<AssignmentSegmentDisplayProps> = (
	props: AssignmentSegmentDisplayProps
) => {
	//console.debug("Making assignment segment display. Props:");
	//console.debug(props);

	//const segment_select_hook=new WrappedHook<string|null>(null);
	//segment_select_hook.side_effects.addSideEffect(props.segment_select_side_effect);

	let display_interval = null;
	if (props.syncer !== undefined) {
		display_interval = props.syncer.interval;
	}
	//const interval_hook = new WrappedHook<OffsetDayTimeInterval|null>(initial_interval);
	if (props.syncer !== undefined) {
		const listener = (new_interval: OffsetDayTimeInterval) => {
			display_interval = new_interval;
		};
		props.syncer.addListener(listener);
	}

	const segment_labels = props.column_meta.translators[2];

	const rows: AssignmentDisplay[][] = [] as AssignmentDisplay[][];
	//let current_row:Assignment[] = rows[0];

	const segments: Segment[] = [];
	for (const node_segment of props.members) {
		const seg: Segment = new Segment(node_segment);
		segments.push(seg);
	}

	if (props.individual_rows) {
		const row: AssignmentDisplay[] = [];
		for (const seg of segments) {
			rows.push([new AssignmentDisplay(null, null, [seg], false)]);
		}
	} else {
		rows.push([new AssignmentDisplay(null, null, segments, false)]);
	}

	console.debug('Built rows', rows);

	const sd_init: SegmentDisplayParameters = {
		rows: rows,
		segment_labels: segment_labels
	};

	const ssd_init: AssignmentSegmentDisplayInit = {
		display_toggler: null,
		display_interval: display_interval,
		segment_select_hook: props.segment_select_hook
	};
	//console.debug("Display interval is "); console.debug(ssd_init.display_interval);

	const ssd = new AssignmentSegmentDisplayConstructor(sd_init, ssd_init);

	const retval = ssd.createReactElement();
	return retval;
};

interface AssignmentSegmentDisplayInit {
	display_toggler: any;
	display_interval: OffsetDayTimeInterval | null;
	segment_select_hook: undefined;
}

class AssignmentSegmentDisplayConstructor extends SegmentDisplayConstructor {
	assignmentseg_init: AssignmentSegmentDisplayInit;

	//;

	constructor(init: SegmentDisplayParameters, assignmentseg_init: AssignmentSegmentDisplayInit) {
		super(init);
		this.assignmentseg_init = assignmentseg_init;
		//this.resize(sdwidth);

		//console.debug("AssignmentSegmentDisplay constructed. Props:");
		//console.debug(this.seg_init);
		//console.debug(this.assignmentseg_init);
	}

	setScale() {
		const interval = this.assignmentseg_init.display_interval;
		if (interval === null) {
			this.autoScale();
		} else {
			console.debug(
				'Setting scale with ' +
					interval.start.moment.toString() +
					' - ' +
					interval.end.moment.toString()
			);
			this.setTimeScaleDirect(interval.start.moment, interval.end.moment);
		}
	}

	_getSizes(width: number): { height: number; left: number; right: number } {
		return {
			height: row_height * (this.getAllRows().length + 1),
			left: width * x_padding_ratio_left,
			right: width * (1.0 - x_padding_ratio_right)
		};
	}

	resize(width: number) {
		const new_sizes = this._getSizes(width);
		this.setScale();

		this.height = new_sizes.height;
		this.left = new_sizes.left;
		this.right = new_sizes.right;
		this.svg_right = new_sizes.right;
		this.segments_right = new_sizes.right;
	}

	getSegmentSelect() {
		let selected_segment_key = null;
		if (this.assignmentseg_init.segment_select_hook !== undefined) {
			//selected_segment_key = this.assignmentseg_init.segment_select_hook.get();
		}
		return selected_segment_key;
	}

	createReactElement() {
		console.debug('AssignmentSegmentDisplay render called.');

		//const element_width=new WrappedHook<number>(0);
		//this.resize(element_width.get());

		//const divref = useRef<{offsetWidth:number}>({offsetWidth:0});

		//this.sizeToContainer(divref,element_width);
		let width = 500;

		let children = [];

		let segment_index: number = 0;

		const selected_segment_key = this.getSegmentSelect();

		console.debug('Selected index is ', selected_segment_key);

		//console.log("rendering segmentdisplay");
		const allrows = this.getAllRows();
		for (let n = 0; n < allrows.length; n++) {
			let row = allrows[n];
			const rect = new RectBounds();
			rect.top = row_height * n;
			rect.bottom = rect.top + row_height - 1;

			for (const assignment_key in row) {
				const assignment = row[assignment_key];
				if (assignment !== null) {
					for (const segment_key in assignment.segments) {
						const segment = assignment.segments[segment_key];
						if (segment != null) {
							const true_segment_key = segment_index.toString();
							let handler = undefined;
							if (this.isInteractive()) {
								handler = (evt: any) => this.segmentInteraction(evt, true_segment_key);
							}
							const highlight = selected_segment_key == true_segment_key;

							rect.left = this.timeToX(segment.start);
							rect.right = this.timeToX(segment.end);

							children.push(
								AssignmentSegmentDisplayConstructor.drawSegmentBox(
									rect,
									segment.type,
									undefined,
									this.getSegmentLabels(),
									segment_index,
									false,
									handler,
									highlight
								)
							);
							//if(this.display_toggler!=null)
							//{
							//	this.display_toggler.configureBox(segment.box, ComponentDisplayToggler.segment_mem, n);
							//}
							children.push(
								drawXLabel(
									rect.left,
									rect.top,
									rect.bottom,
									segment.start.getTime(),
									'black',
									true,
									segment_index + '_xL'
								)
							);
							children.push(
								drawXLabel(
									rect.right,
									rect.top,
									rect.bottom,
									segment.end.getTime(),
									'black',
									false,
									segment_index + '_xR'
								)
							);

							segment_index++;
						}
					}
				}
			}
		}

		children = children.concat(this._drawDayBoxes());

		if (children.length == 0) {
			children.push(
				drawTextBox(
					new RectBounds(0, 0, width, this.height),
					'red',
					'No Children',
					'black',
					undefined,
					undefined,
					undefined,
					segment_index++
				)
			);
		}

		return (
			/*
			<Box ref={divref} width="100%" height={this.height}>
				<ReactSVG
					bounds={new RectBounds(0,0,element_width.get(),this.height)}
					>
					{children}
				</ReactSVG>
			</Box>
			*/
			//TODO
			null
		);
	}

	isInteractive() {
		const retval = this.assignmentseg_init.segment_select_hook !== undefined;
		console.debug('Is interactive:', retval);
		return retval;
	}

	segmentInteraction(evt: any, interacted_segment_key: string) {
		if (!this.isInteractive()) {
			return;
		}

		let new_selected_segment_key = null;
		switch (evt.type) {
			case 'click':
				if (interacted_segment_key != this.assignmentseg_init.segment_select_hook.get()) {
					new_selected_segment_key = interacted_segment_key;
				}
				break;
			case 'mouseenter':
			case 'mouseleave':
			default:
				return;
		}

		this.assignmentseg_init.segment_select_hook.set(new_selected_segment_key);
	}
}
