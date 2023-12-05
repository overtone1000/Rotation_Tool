import { derived, writable } from 'svelte/store';
import type { OperationContents } from '../commons/refactored/ajax/commands_generic';
import { processStagingData01Dates } from '../commons/refactored/staging/data_processing/processing01';
import { processStagingData02Rendering } from '../commons/refactored/staging/data_processing/processing02';
import { processStagingData03ProposedChanges } from '../commons/refactored/staging/data_processing/processing03';
import {
	StagingTypes,
	type ASStagingResponseMessage
} from '../commons/refactored/staging/data_processing/stagingdata';
import { StagingSelectionMode } from '../commons/refactored/staging/members/highlighting';
import type { RenderedAssignable } from '../commons/refactored/staging/members/rendered_assignable';
import type { GenericRenderedConstraint } from '../commons/refactored/staging/members/rendered_constraint';
import { RightPanelContext, type ProposedAddition } from '../commons/refactored/staging/staging';

export interface StagingOperationContents extends OperationContents {
	update_data: ASStagingResponseMessage;
}
export const stagingInit = writable<StagingOperationContents | undefined>(undefined);

export const stagingContext = writable<RightPanelContext>(RightPanelContext.edit);
export const stagingSelectedDate = writable<Date>(new Date());
export const stagingProposedAddition = writable<ProposedAddition | undefined>(undefined);

export const stagingProcessedDates = derived(stagingInit, ($stagingInit) => {
	if (!$stagingInit) {
		return undefined;
	}
	//console.debug('Processing dates', $stagingInit);
	const retval = processStagingData01Dates($stagingInit);
	//console.debug('Processed dates:', retval);
	return retval;
});

export const stagingProcessedMembers = derived(stagingProcessedDates, ($stagingProcessedDates) => {
	if (!$stagingProcessedDates) {
		return undefined;
	}
	//console.debug('Processing dates', $stagingProcessedDates);
	const retval = processStagingData02Rendering($stagingProcessedDates);
	//console.debug('Processed members', retval);
	return retval;
});

export const stagingProcessedProposal = derived(
	[stagingProcessedMembers, stagingSelectedDate, stagingProposedAddition],
	([$stagingProcessedMembers, $stagingSelectedDate, $stagingProposedAddition]) => {
		if (!$stagingProcessedMembers) {
			return undefined;
		}
		/*
		console.debug('Processing dates', [
			$stagingProcessedMembers,
			$stagingSelectedDate,
			$stagingProposedAddition
		]);
		*/
		const retval = processStagingData03ProposedChanges(
			$stagingProcessedMembers,
			$stagingSelectedDate,
			$stagingProposedAddition
		);
		//console.debug('Processed proposal', retval);
		return retval;
	}
);

export type Selectable = RenderedAssignable | GenericRenderedConstraint;
type Selections = Map<StagingSelectionMode, Set<Selectable>>;
export interface SelectionStore {
	selections: Selections;
	//mode:StagingSelectionMode,
	handleClick: (m: Selectable, event: MouseEvent) => void;
}
export interface SelectionConfig {
	mode: StagingSelectionMode;
	multi: boolean;
}
const createSelectionStore = () => {
	const stagingSelection = writable<Selections>(new Map<StagingSelectionMode, Set<Selectable>>());
	const selectionConfigStack = writable<SelectionConfig[]>([
		{ mode: StagingSelectionMode.primary_selected, multi: false }
	]);

	const getGroup = (mode: StagingSelectionMode, selections: Selections) => {
		let retval = selections.get(mode);
		if (!retval) {
			retval = new Set<Selectable>();
			selections.set(mode, retval);
		}
		return retval;
	};

	const toggleSelection = (m: Selectable, multi: boolean, selection_mode: StagingSelectionMode) => {
		//console.debug('Toggle selection', m, multi, selection_mode);
		if (multi) {
			stagingSelection.update((current) => {
				const group = getGroup(selection_mode, current);
				if (group.has(m)) {
					//console.debug('Deleting', m);
					group.delete(m);
					m.updateHighlighting(StagingSelectionMode.none);
				} else {
					//console.debug('Adding', m);
					group.add(m);
					m.updateHighlighting(selection_mode);
				}
				return current;
			});
		} else {
			stagingSelection.update((current) => {
				const group = getGroup(selection_mode, current);
				for (const member of group) {
					member.updateHighlighting(StagingSelectionMode.none);
				}
				if (group.has(m)) {
					group.clear();
				} else {
					group.clear();
					group.add(m);
					m.updateHighlighting(selection_mode);
				}
				return current;
			});
		}
	};

	const handleClick = (m: Selectable, event: MouseEvent, selection_config: SelectionConfig) => {
		//console.debug('Handle click', m, event, selection_config);
		let multi = false;
		if (selection_config.multi || event.ctrlKey) {
			multi = true;
		}
		switch (selection_config.mode) {
			case StagingSelectionMode.commit:
				return;
			case StagingSelectionMode.none:
				return;
			case StagingSelectionMode.secondary_selected:
				if (m.getStagingType() == StagingTypes.Assignable) {
					toggleSelection(m, multi, selection_config.mode);
				}
				break;
			case StagingSelectionMode.primary_selected:
			case StagingSelectionMode.proposed:
			default:
				toggleSelection(m, multi, selection_config.mode);
				break;
		}
	};

	const clearSelectionMode = (mode: StagingSelectionMode) => {
		//console.debug('Selection mode clear', mode);
		stagingSelection.update((selections) => {
			const group = getGroup(mode, selections);
			group.forEach((selectable) => {
				selectable.updateHighlighting(StagingSelectionMode.none);
			});
			group.clear();
			return selections;
		});
	};

	const clearSelection = () => {
		//console.debug('Full selection clear');
		stagingSelection.update((selections) => {
			selections.forEach((selectable_group) => {
				selectable_group.forEach((selectable) => {
					selectable.updateHighlighting(StagingSelectionMode.none);
				});
			});
			selections.clear();
			return selections;
		});
	};

	const setSecondaryConfig = (config: SelectionConfig) => {
		//console.debug('Secondary config set', config);
		selectionConfigStack.update((current) => {
			if (current.length > 1) {
				current = current.slice(0, 1);
			}
			current.push(config);
			return current;
		});
	};

	const revertToPrimaryConfig = () => {
		//console.debug('Primary mode revert');
		selectionConfigStack.update((configstack) => {
			if (configstack.length > 1) {
				clearSelectionMode(configstack[configstack.length - 1].mode);
				configstack = configstack.slice(0, 1);
			}
			return configstack;
		});
	};

	const setBaseMode = (new_config: SelectionConfig) => {
		//console.debug('Base mode set', new_config);
		clearSelection();
		selectionConfigStack.update(() => {
			return [new_config];
		});
	};

	const setSelection = (mode: StagingSelectionMode, selection: Set<Selectable>) => {
		//console.debug('Selection set', mode, selection);
		stagingSelection.update((current) => {
			const group = getGroup(mode, current);
			group.forEach((member) => {
				member.updateHighlighting(StagingSelectionMode.none);
			});
			selection.forEach((member) => {
				member.updateHighlighting(mode);
			});
			current.set(mode, selection);
			return current;
		});
	};

	const selectionStore = derived(
		[stagingSelection, selectionConfigStack],
		([selections, configstack]) => {
			//console.debug('Selection store', selections, configstack);
			const current_config = configstack[configstack.length - 1];
			const retval: SelectionStore = {
				selections: selections,
				handleClick: (m: Selectable, event: MouseEvent) => {
					return handleClick(m, event, current_config);
				}
			};
			////console.debug("Updating selection store",retval,selections,modestack);
			return retval;
		}
	);

	return {
		subscribe: selectionStore.subscribe,
		clearHighlighting: clearSelection,
		setSecondaryConfig: setSecondaryConfig,
		revertToPrimaryConfig: revertToPrimaryConfig,
		setBaseMode: setBaseMode,
		setSelection: setSelection
	};
};
export const stagingSelection = createSelectionStore();
