import { Box } from '@mui/material';
import { max, min } from 'date-fns';
import React, { FC, useRef } from 'react';
import { DataMeta } from '../data_processing/data_types';
import { AssignmentMember } from '../data_processing/extended_types/bndata/AssignmentMember';
import {
	Constraint,
	ConstraintClass,
	MatchOne,
	SingleWorker
} from '../data_processing/extended_types/bndata/Constraint';
import { ScheduleTemplateDetails } from '../data_processing/extended_types/bndata/ScheduleTemplateDetails';
import { AssignmentSegmentType, Segment } from '../data_processing/extended_types/bndata/Segmemt';
import { AssignmentType } from '../data_processing/extended_types/id_tables/AssignmentTypeTable';
import { WrappedHook } from '../react/WrappedHook';
import { PointString } from '../svg/PointString';
import { ReactSVG } from '../svg/ReactSVG';
import { RectBounds } from '../svg/RectBounds';
import { drawReactPolyline, drawText } from '../svg/functions';
import { autoscheda_theme } from '../theming/theme';
import {
	AssignmentDisplay,
	SegmentDisplayConstructor,
	SegmentDisplayParameters,
	constraint_col,
	row_height,
	x_padding_ratio_left,
	x_padding_ratio_right
} from './SegmentDisplay';

export interface ScheduleTemplateDisplayProps {
	key: string;
	data: ScheduleTemplateDetails;
	column_meta: DataMeta;
	table_meta: DataMeta;
	assignment_select_hook?: undefined | WrappedHook<AssignmentSelectionState>;
	constraint_select_hook?: undefined | WrappedHook<ConstraintSelectionState>;
}

const getSegments = (type: AssignmentType, am: AssignmentMember) => {
	let tt_details = type.getDetails();

	const segments: Segment[] = [];
	for (const node_segment of tt_details.getNodeSegments()) {
		const seg: Segment = new Segment(node_segment, am.getDayOffset());
		segments.push(seg);
	}

	return segments;
};

export const ScheduleTemplateDisplay: FC<ScheduleTemplateDisplayProps> = (
	props: ScheduleTemplateDisplayProps
) => {
	console.debug('Making schedule template display. Props:');
	console.debug(props);

	//Get assignment types
	const assignment_type_map: { [i: number]: AssignmentType } = {};
	const ttypes = props.table_meta.assignment_types.rows;
	for (const row_key in ttypes) {
		let this_tt = new AssignmentType(ttypes[row_key]);
		assignment_type_map[this_tt.getID()] = this_tt;
	}

	//const assignment_select_hook=new WrappedHook<AssignmentDisplayState>({interacted_assignment_index:undefined,selected:false});
	//assignment_select_hook.side_effects.addSideEffect(props.assignment_select_handler);

	//const constraint_select_hook=new WrappedHook<ConstraintDisplayState>({active_constraint_index:undefined, selected:false, diagrams_shown:false});
	//constraint_select_hook.side_effects.addSideEffect(props.constraint_select_handler);

	const assignment_members = props.data.getAssignmentMembers();
	const constraint_members = props.data.getConstraintMembers();

	const segment_labels = props.column_meta.translators[2];

	const rows: [AssignmentDisplay[]] = [[]] as [AssignmentDisplay[]];

	console.debug('rows instantiated, length is ' + rows.length.toString());
	//let current_row:Assignment[] = rows[0]; //Only one assignment per row...

	for (const index in assignment_members) {
		const assignment_member = assignment_members[index];

		const Assignment_type: AssignmentType =
			assignment_type_map[assignment_member.getAssignmentTypeID()];

		if (!Assignment_type) {
			console.error("Couldn't discover assignment type", assignment_member);
		} else {
			const this_assignment: AssignmentDisplay = new AssignmentDisplay(
				Assignment_type.getName(),
				assignment_member,
				getSegments(Assignment_type, assignment_member),
				false
			);

			if (rows[rows.length - 1] === null || rows[rows.length - 1].length == 0) {
				rows[rows.length - 1] = [this_assignment];
			} else {
				rows.push([this_assignment]);
			}
		}
	}

	const sd_init: SegmentDisplayParameters = {
		rows: rows,
		segment_labels: segment_labels
	};

	let assignment_select_hook = props.assignment_select_hook;
	if (assignment_select_hook === undefined) {
		assignment_select_hook = new WrappedHook<AssignmentSelectionState>(null);
	}

	let constraint_select_hook = props.constraint_select_hook;
	if (constraint_select_hook === undefined) {
		constraint_select_hook = new WrappedHook<ConstraintSelectionState>(null);
	}

	const sgd_init: ScheduleTemplateSegmentDisplayInit = {
		constraints: constraint_members,
		assignment_select_hook: assignment_select_hook,
		constraint_select_hook: constraint_select_hook
	};

	const sgd = new ScheduleTemplateSegmentDisplayConstructor(sd_init, sgd_init);

	const retval = sgd.createReactElement(assignment_type_map);
	return retval;
};

export type AssignmentSelectionState = string | null;
type AssignmentInteractionState = string | null;
export type ConstraintSelectionState = string | null;
type ConstraintInteractionState = string | null;
/*
export interface AssignmentSelectionState
{
	selected_assignment_index:string|null
}

interface AssignmentInteractionState
{
	interacted_assignment_index:string
}

export interface ConstraintSelectionState
{
	selected_constraint_index:string|null
}

interface ConstraintInteractionState
{
	interacted_constraint_index:string
	//diagrams_shown:boolean
}
*/

interface DrawnSegment {
	leftx: number;
	rightx: number;
	type: AssignmentSegmentType;
}

interface TopBottom {
	top: number;
	bottom: number;
	lerp_top: number;
	lerp_bottom: number;
}

interface DrawnAssignment {
	assignment: AssignmentDisplay;
	drawn_segments: DrawnSegment[];
	leftx: number;
	rightx: number;
	global_leftx: number;
	global_rightx: number;
	row_index: number;
	top_bottom: TopBottom;
}

const constraint_text_width = 200;
const diagram_width = 50;

interface ScheduleTemplateSegmentDisplayInit {
	constraints: Constraint[];
	assignment_select_hook: WrappedHook<AssignmentSelectionState>;
	constraint_select_hook: WrappedHook<ConstraintSelectionState>;
}

class ScheduleTemplateSegmentDisplayConstructor extends SegmentDisplayConstructor {
	sgseg_init: ScheduleTemplateSegmentDisplayInit;

	assignment_interaction_hook: WrappedHook<AssignmentInteractionState>;
	constraint_interaction_hook: WrappedHook<ConstraintInteractionState>;

	diagram_right: number;
	segments_right: number = null;

	constructor(init: SegmentDisplayParameters, sgseg_init: ScheduleTemplateSegmentDisplayInit) {
		super(init);
		this.sgseg_init = sgseg_init;

		this.assignment_interaction_hook = new WrappedHook<AssignmentInteractionState>(null);
		this.constraint_interaction_hook = new WrappedHook<ConstraintInteractionState>(null); //, diagrams_shown:true});

		this.resize(0);

		console.debug('ScheduleTemplateSegmentDisplay constructed.');
	}

	resize(width: number) {
		this.height = row_height * (this.getAllRows().length + 1);
		this.left = width * x_padding_ratio_left;
		this.right = width * (1.0 - x_padding_ratio_right);
		this.svg_right = this.right;

		if (this.sgseg_init.constraints != null && this.sgseg_init.constraints.length > 0) {
			this.diagram_right = this.right - constraint_text_width;
			this.segments_right = this.diagram_right - diagram_width;
		} else {
			this.diagram_right = this.right;
			this.segments_right = this.right;
		}
	}

	getAssignmentSelect() {
		if (this.sgseg_init.assignment_select_hook !== undefined) {
			return this.sgseg_init.assignment_select_hook.get();
		} else {
			return null;
		}
	}

	getConstraintSelect() {
		if (this.sgseg_init.constraint_select_hook !== undefined) {
			return this.sgseg_init.constraint_select_hook.get();
		} else {
			return null;
		}
	}

	private getRowTopBottom = function (row_index: number): TopBottom {
		let retval: TopBottom = {} as TopBottom;

		retval.top = row_height * row_index;
		retval.bottom = retval.top + row_height - 1;

		const in_lerp = 1.0 / 3.0;
		retval.lerp_top = retval.bottom * in_lerp + retval.top * (1 - in_lerp);
		retval.lerp_bottom = retval.bottom * (1 - in_lerp) + retval.top * in_lerp;

		return retval;
	};

	private drawAssignment = (assignment: AssignmentDisplay, row: number) => {
		let minmoments = new Array();
		let maxmoments = new Array();

		const drawn_assignment: DrawnAssignment = {} as DrawnAssignment;
		drawn_assignment.assignment = assignment;
		drawn_assignment.row_index = row;
		drawn_assignment.top_bottom = this.getRowTopBottom(row);
		drawn_assignment.drawn_segments = [];

		for (const segment of assignment.segments) {
			if (segment !== null) {
				//console.log("new segment...");
				//console.debug(segment);

				let leftm = segment.start.moment;
				let rightm = segment.end.moment;

				minmoments.push(leftm);
				maxmoments.push(rightm);

				let drawn_segment: DrawnSegment = {
					leftx: this.momentToX(leftm),
					rightx: this.momentToX(rightm),
					type: segment.type
				};

				drawn_assignment.drawn_segments.push(drawn_segment);

				//draw_xlabel(this.svg, leftx, rect_bottom, rect_top, segment.start_time, true);
				//draw_xlabel(this.svg, rightx, rect_bottom, rect_top, segment.end_time, false);
			}
		}

		let first_date = min(minmoments);
		let last_date = max(maxmoments);

		drawn_assignment.leftx = this.momentToX(first_date);
		drawn_assignment.rightx = this.momentToX(last_date);

		drawn_assignment.global_leftx = drawn_assignment.leftx;
		drawn_assignment.global_rightx = drawn_assignment.rightx;

		console.debug('Boundaries for assignment set.', first_date, last_date, drawn_assignment);

		return drawn_assignment;
	};

	createReactElement(assignment_type_map: { [i: number]: AssignmentType }): React.ReactElement {
		const element_width = new WrappedHook<number>(0);

		const divref = useRef<{ offsetWidth: number }>({ offsetWidth: 0 });

		const children: React.ReactNode[][] = [] as React.ReactNode[][];
		for (
			let n = 0;
			n < 4;
			n++ //four layers
		) {
			children.push([] as React.ReactNode[]);
		}

		//const diagrams = new Array();

		let assignments: AssignmentDisplay[] = [];
		const drawn_assignments: DrawnAssignment[] = [];

		//console.log("rendering scheduletemplatesegmentdisplay");
		let key: number = 0;

		const assignment_select_state = this.getAssignmentSelect();
		const constraint_select_state = this.getConstraintSelect();

		const assignment_interaction_state = this.assignment_interaction_hook.get();
		const constraint_interaction_state = this.constraint_interaction_hook.get();

		//Collect base assignemnts
		for (let n = 0; n < this.getInitRows().length; n++) {
			let row = this.getInitRows()[n];
			assignments = assignments.concat(row);
		}

		//Add ghost assignments
		const additional_rows = [] as AssignmentDisplay[][];
		if (this.sgseg_init.constraints != null) {
			for (const constraint_index in this.sgseg_init.constraints) {
				const constraint = this.sgseg_init.constraints[constraint_index];

				switch (constraint.getType()) {
					case ConstraintClass.MatchOne:
						{
							const details = constraint.details as MatchOne;
							details.build(assignments);
							for (const unavailable of details.getUnavailableAssignmentMembers()) {
								const type = assignment_type_map[unavailable.getAssignmentTypeID()];
								const assignment = new AssignmentDisplay(
									type.getName(),
									unavailable,
									getSegments(type, unavailable),
									true
								);
								assignments.push(assignment);
								additional_rows.push([assignment]);
							}
						}
						break;
					case ConstraintClass.SingleWorker:
						break;
				}
			}
		}
		this.setAdditionalRows(additional_rows);

		//Now that any additional rows have been added, resize
		this.resize(element_width.get());
		this.sizeToContainer(divref, element_width);
		this.autoScale();

		//Now draw assignments
		for (const assignment_index in assignments) {
			const assignment = assignments[assignment_index];
			const drawn_assignment = this.drawAssignment(assignment, parseInt(assignment_index));
			drawn_assignments.push(drawn_assignment);
		}

		//Now draw assignment segments
		for (const assignment_index in drawn_assignments) {
			const drawn_assignment = drawn_assignments[assignment_index];
			const handler = (evt: any) => this.assignmentInteraction(evt, assignment_index);

			for (const drawn_segment of drawn_assignment.drawn_segments) {
				const rect = new RectBounds();
				rect.top = drawn_assignment.top_bottom.top;
				rect.bottom = drawn_assignment.top_bottom.bottom;
				rect.left = drawn_segment.leftx;
				rect.right = drawn_segment.rightx;

				let final_handler = undefined;
				if (!drawn_assignment.assignment.isGhost()) {
					final_handler = handler;
				}

				children[1].push(
					SegmentDisplayConstructor.drawSegmentBox(
						rect,
						drawn_segment.type,
						true,
						this.getSegmentLabels(),
						key++,
						drawn_assignment.assignment.isGhost(),
						final_handler,
						false
					)
				);
			}

			const rect2 = new RectBounds(
				drawn_assignment.leftx,
				drawn_assignment.top_bottom.lerp_top,
				drawn_assignment.rightx,
				drawn_assignment.top_bottom.lerp_bottom
			);
			let text_color: string;
			if (drawn_assignment.assignment.isGhost()) {
				text_color = 'white';
			} else {
				text_color = 'black';
			}
			children[1].push(
				drawText(rect2, text_color, drawn_assignment.assignment.name, undefined, key++)
			);

			//draw outline around the assignments
			let highlight = false;
			if (assignment_select_state == assignment_index) {
				//assignment_iteraction_state.interacted_assignment_index==assignment_index &&
				highlight = true;
			}
			const rect = new RectBounds(
				drawn_assignment.global_leftx,
				drawn_assignment.top_bottom.top,
				drawn_assignment.global_rightx,
				drawn_assignment.top_bottom.bottom
			);
			const ps = new PointString();
			ps.makeBox(rect);
			if (highlight) {
				children[3].push(drawReactPolyline(ps, 'yellow', false, 5, 'ol' + key++));
			} else {
				children[2].push(drawReactPolyline(ps, 'black', false, 1, 'ol' + key++));
			}
		}

		let constraint_label_flags = new Array(drawn_assignments.length);
		for (const n in constraint_label_flags) {
			constraint_label_flags[n] = false;
		}

		const get_label_row = (first_choice: number, last_choice: number) => {
			for (let n = first_choice; n <= last_choice; n++) {
				if (!constraint_label_flags[n]) {
					constraint_label_flags[n] = true;
					return n;
				}
			}

			for (let n = first_choice - 1; n >= 0; n--) {
				if (!constraint_label_flags[n]) {
					constraint_label_flags[n] = true;
					return n;
				}
			}

			for (let n = last_choice + 1; n < this.getInitRows().length; n++) {
				if (!constraint_label_flags[n]) {
					constraint_label_flags[n] = true;
					return n;
				}
			}

			this.getInitRows().push();
			constraint_label_flags.push(true);
			return this.getInitRows().length - 1;
		};

		//Draw diagrams
		if (this.sgseg_init.constraints != null) {
			//Drawing steps
			const no_interaction_or_selection =
				assignment_select_state == null &&
				constraint_select_state == null &&
				assignment_interaction_state == null &&
				constraint_interaction_state == null;

			for (const constraint_index in this.sgseg_init.constraints) {
				let constraint = this.sgseg_init.constraints[constraint_index];

				const handler = (evt: any) => this.constraintInteraction(evt, constraint_index);

				let highlight: boolean = false;
				let drawdiagrams: boolean = false;

				if (no_interaction_or_selection) {
					drawdiagrams = true;
				} else if (constraint_select_state == constraint_index) {
					highlight = true;
					drawdiagrams = true;
				} else if (constraint_interaction_state == constraint_index) {
					drawdiagrams = true;
				} else if (assignment_select_state !== null || assignment_interaction_state !== null) {
					for (const i of constraint.details.getInvolvedAssignments(assignments)) {
						if (
							i.toString() == assignment_select_state ||
							i.toString() == assignment_interaction_state
						) {
							drawdiagrams = true;
							break;
						}
					}
				}

				let constraint_children_container: React.ReactNode[];
				if (drawdiagrams) {
					constraint_children_container = children[3];
				} else {
					constraint_children_container = children[1];
				}

				switch (constraint.getType()) {
					case ConstraintClass.MatchOne:
						{
							const details = constraint.details as MatchOne;
							const label_row = get_label_row(
								details.getToMatchAssignmentMemberIndex(),
								details.getToMatchAssignmentMemberIndex()
							);
							const label_assignment = drawn_assignments[label_row];

							const rect = new RectBounds(
								this.diagram_right,
								label_assignment.top_bottom.top,
								this.right,
								label_assignment.top_bottom.bottom
							);

							constraint_children_container.push(
								SegmentDisplayConstructor.drawSegmentBox(
									rect,
									constraint_col,
									true,
									this.getSegmentLabels(),
									key++,
									false,
									handler,
									highlight
								)
							);
							constraint_children_container.push(
								drawText(rect, 'black', 'Match One', undefined, key++)
							);

							let assignment_as_array = new Array();
							assignment_as_array.push(details.getToMatchAssignmentMemberIndex());

							if (drawdiagrams) {
								constraint_children_container.push(
									this.drawDiagram(
										label_assignment,
										drawn_assignments,
										assignment_as_array,
										0.5,
										0.25,
										0.25,
										false,
										(key++).toString()
									)
								);
								constraint_children_container.push(
									this.drawDiagram(
										label_assignment,
										drawn_assignments,
										details.getCandidatesAndUAMs(),
										0.75,
										0.75,
										0.75,
										true,
										(key++).toString()
									)
								);
							}
						}
						break;
					case ConstraintClass.SingleWorker:
						{
							const details = constraint.details as SingleWorker;
							const assignments = details.getAssignments();
							const label_row = get_label_row(assignments[0], assignments[assignments.length - 1]);
							const label_assignment = drawn_assignments[label_row];

							const rect = new RectBounds(
								this.diagram_right,
								label_assignment.top_bottom.top,
								this.right,
								label_assignment.top_bottom.bottom
							);
							constraint_children_container.push(
								SegmentDisplayConstructor.drawSegmentBox(
									rect,
									constraint_col,
									true,
									this.getSegmentLabels(),
									key++,
									false,
									handler,
									highlight
								)
							);
							constraint_children_container.push(
								drawText(rect, 'black', 'Single Worker', undefined, key++)
							);

							if (drawdiagrams) {
								constraint_children_container.push(
									this.drawDiagram(
										label_assignment,
										drawn_assignments,
										assignments,
										0.25,
										0.5,
										0.25,
										false,
										(key++).toString()
									)
								);
							}
						}
						break;
				}
			}
		}

		children[0].push(this._drawDayBoxes());

		return (
			<Box ref={divref} key="sg_seg_disp" width="100%" height={this.height}>
				<ReactSVG bounds={new RectBounds(0, 0, element_width.get(), this.height)}>
					{children}
				</ReactSVG>
			</Box>
		);
	}

	assignmentInteraction(evt: any, assignment_index: string) {
		if (this.sgseg_init.assignment_select_hook === undefined) {
			return;
		}

		const current_selection_state = this.getAssignmentSelect();

		if (current_selection_state !== null && evt.type != 'click') {
			return; //if there is a selected constraint, ignore all interactions but clicks of other constraints
		}

		const current_interaction_state: AssignmentInteractionState =
			this.assignment_interaction_hook.get();

		switch (evt.type) {
			case 'mouseenter':
				this.assignment_interaction_hook.set(assignment_index);
				break;
			case 'click':
				if (current_selection_state != assignment_index) {
					this.sgseg_init.assignment_select_hook.set(assignment_index);
				} else {
					this.sgseg_init.assignment_select_hook.set(null);
				}
				this.assignment_interaction_hook.set(assignment_index);
				break;
			case 'mouseleave':
				this.assignment_interaction_hook.set(null);
			default:
				break;
		}
	}

	constraintInteraction(evt: any, constraint_index: string) {
		if (this.sgseg_init.constraint_select_hook === undefined) {
			return;
		}

		const current_selection_state = this.getConstraintSelect();

		if (current_selection_state !== null && evt.type != 'click') {
			return; //if there is a selected constraint, ignore all interactions but clicks of other constraints
		}

		const current_interaction_state: ConstraintInteractionState =
			this.constraint_interaction_hook.get();

		switch (evt.type) {
			case 'mouseenter':
				this.constraint_interaction_hook.set(constraint_index);
				break;
			case 'click':
				if (current_selection_state != constraint_index) {
					this.sgseg_init.constraint_select_hook.set(constraint_index);
				} else {
					this.sgseg_init.constraint_select_hook.set(null);
				}
				this.constraint_interaction_hook.set(constraint_index);
				break;
			case 'mouseleave':
				this.constraint_interaction_hook.set(null);
			default:
				break;
		}
	}

	drawDiagram(
		label_assignment: DrawnAssignment,
		drawn_assignments: DrawnAssignment[],
		associated_assignment_indices: number[],
		x_frac: number,
		y_frac: number,
		row_y_frac: number,
		dashed: boolean,
		key: string
	): React.ReactNode[] {
		let lines: React.ReactNode[] = [];

		let subkey: number = 0;

		let ps = new PointString();
		let x = this.diagram_right * (1.0 - x_frac) + this.segments_right * x_frac;
		let y =
			label_assignment.top_bottom.bottom * y_frac +
			label_assignment.top_bottom.top * (1.0 - y_frac);
		ps.add(this.diagram_right, y);
		ps.add(x, y);
		lines.push(
			drawReactPolyline(
				ps,
				autoscheda_theme.palette.common.white,
				dashed,
				undefined,
				'dg' + key + ':' + (subkey++).toString()
			)
		);

		//let solid_y_min = 9999999999;
		//let solid_y_max = -9999999999;

		let solid_y_min = y;
		let solid_y_max = y;

		console.debug('Associated assignment indices are ', associated_assignment_indices);
		if (associated_assignment_indices != null) {
			for (const assignment_index of associated_assignment_indices) {
				let drawn_assignment = drawn_assignments[assignment_index];
				if (drawn_assignment == null) {
					console.debug(
						'null drawn assignment',
						drawn_assignments,
						assignment_index,
						associated_assignment_indices
					);
				}
				let y2 =
					drawn_assignment.top_bottom.bottom * row_y_frac +
					drawn_assignment.top_bottom.top * (1.0 - row_y_frac);

				if (y2 > solid_y_max) {
					solid_y_max = y2;
				}
				if (y2 < solid_y_min) {
					solid_y_min = y2;
				}
				ps.clear();
				ps.add(x, y2);
				ps.add(drawn_assignment.global_rightx, y2);
				lines.push(
					drawReactPolyline(
						ps,
						autoscheda_theme.palette.common.white,
						dashed,
						undefined,
						'dg' + key + ':' + (subkey++).toString()
					)
				);
			}
		}

		ps.clear();
		ps.add(x, solid_y_max);
		ps.add(x, solid_y_min);
		lines.push(
			drawReactPolyline(
				ps,
				autoscheda_theme.palette.common.white,
				dashed,
				undefined,
				'dg' + key + ':' + (subkey++).toString()
			)
		);

		/*
		diagram.show = function()
		{
			for(const line of lines)
			{
				line.setAttribute("visibility","");
			}
		}

		diagram.hide = function()
		{
			for(const line of lines)
			{
				line.setAttribute("visibility","hidden");
			}
		}
		*/

		//this.diagrams.push(diagram);

		return lines;
	}
}
