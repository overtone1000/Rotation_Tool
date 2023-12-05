import type { ConstraintClass } from '../../needs_refactoring/data_processing/extended_types/bndata/Constraint';
import type { ProcessingResult02 } from './data_processing/processing02';
import type { AddType } from './data_processing/processing03';
import type { StagingSelectionMode } from './members/highlighting';
import type { RenderedAssignable } from './members/rendered_assignable';
import type { GenericRenderedConstraint } from './members/rendered_constraint';

export const table_item_height = 38;
export type InteractionHandler = (interation: Interaction, details?: InteractionDetails) => void;

export enum RightPanelContext {
	add,
	edit,
	commit
}

export enum Interaction {
	//forceReprocess,
	clickStagingMember,
	clearPrimarySelection,
	selectDate,
	changeProposedAddition,
	selectConstraintMember,
	changeFocusedSelection,
	releaseFocusedSelection,
	changeSecondarySelection
}

export enum ForceReprocess {
	Process01,
	Process02,
	Process03
}

export interface SelectInteraction {
	selection: (RenderedAssignable | GenericRenderedConstraint)[];
	multi: boolean;
}

export interface ChangeFocusedSelectionInteraction {
	handler: SelectInterface;
	initial_selections: SelectionHighlighting;
}

export interface DateClick {
	date: Date;
	multi: boolean;
}

export type InteractionDetails =
	| ForceReprocess
	| SelectInteraction
	| DateClick
	| ProposedAddition
	| ChangeFocusedSelectionInteraction
	| SelectionHighlighting;

export interface ProposedAddition {
	//selected_date:Date, //Source of truth for this needs to be in the main staging component
	context: AddType;
	selected_type: number | ConstraintClass;
	constraint?: GenericRenderedConstraint;
	multiple: number;
}

export type SelectionHandler = (
	selection_details: SelectInteraction,
	preprocessing: ProcessingResult02
) => void;
export interface SelectInterface {
	invert: SelectionHandler;
}
/*
export class SingleSelect extends WrappedHook<number[]> implements SelectInterface
{
  constructor(index:number[])
  {
    super(index);
  }
  update:SelectionHandler = (selection_details:SelectInteraction) =>
  {
    const new_selection:number[]=[];
    for(const key in selection_details.selection)
    {
      new_selection.push(selection_details.selection[key].getIndex());
    }
    this.set(new_selection);
  }
}
*/

export interface SelectionHighlighting {
	highlighting: StagingSelectionMode;
	indices: Set<number>;
}

export interface StagingSelectionContextContents {
	selectedStaging: [];
	secondarySelections: SelectionHighlighting;
}
export function genericMultiselectInvert<T>(multi: boolean, invert_set: Set<T>, old_set: Set<T>) {
	const new_set = new Set<T>(old_set);
	console.debug('invert', invert_set, old_set);

	if (old_set !== null && old_set.size == 1 && invert_set.size == 1) {
		const v1 = old_set.values().next().value;
		const v2 = invert_set.values().next().value;
		console.debug(v1, v2);
		if (v1 === v2) {
			return new Set<T>();
		}
	}

	if (multi === undefined || multi === false) {
		new_set.clear();
	}

	for (const index of invert_set) {
		if (new_set.has(index)) {
			new_set.delete(index);
		} else {
			new_set.add(index);
		}
	}

	//console.debug("Multi final is",new_set);
	return new_set;
}
