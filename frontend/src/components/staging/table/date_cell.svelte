<script lang="ts">
	import Button from '@smui/button';
	import { Label } from '@smui/tab';
	import Tooltip, { Wrapper } from '@smui/tooltip';
	import { toDateString_ShortDisplay } from '../../commons/refactored/commons/Dates';
	import { localDateToEpochDay } from '../../commons/refactored/staging/data_processing/processing01';
	import { StagingSelectionMode } from '../../commons/refactored/staging/members/highlighting';
	import { RightPanelContext } from '../../commons/refactored/staging/staging';
	import type { SummaryNode } from '../../commons/refactored/staging/week_sorting';
	import {
		stagingContext,
		stagingProcessedDates,
		stagingProcessedMembers,
		stagingSelectedDate,
		stagingSelection,
		type Selectable
	} from '../stores';
	import './table.css';

	export let date: Date | null;

	const perfect_chip = 'green';
	const warning_chip = 'goldenrod';
	const error_chip = 'red';

	let assigned_color: string = warning_chip;
	let required_color: string = perfect_chip;

	let worker_range_string: string;

	let required_worker_tooltip: string;
	let assigned_worker_tooltip: string;

	let summary: SummaryNode | undefined;
	$: {
		summary = undefined;
		if (date) {
			const epoch_day = localDateToEpochDay(date);
			if (epoch_day) {
				summary = $stagingProcessedDates?.response.update_data.data.summaries[epoch_day];
			}

			if (summary) {
				if (summary.worker_assigned == summary.worker_active) {
					assigned_color = perfect_chip;
				} //Perfect!

				if (summary.worker_min > summary.worker_active) {
					required_color = error_chip;
				} //Not enough workers to cover assignments!
				else if (summary.worker_max < summary.worker_active) {
					required_color = warning_chip;
				} //Not enough assignments for every worker!

				if (summary.worker_min == summary.worker_max) {
					worker_range_string = summary.worker_min.toString();
				} else {
					worker_range_string = summary.worker_min + '-' + summary.worker_max;
				}

				let worker_range_noun: string;
				let worker_active_noun: string;
				if (summary.worker_max > 1) {
					worker_range_noun = ' workers';
				} else {
					worker_range_noun = ' worker';
				}

				if (summary.worker_active > 1) {
					worker_active_noun = ' workers';
				} else {
					worker_active_noun = ' worker';
				}

				required_worker_tooltip =
					worker_range_string + worker_range_noun + ' required for this day.';
				assigned_worker_tooltip =
					summary.worker_assigned +
					' of ' +
					summary.worker_active +
					' active' +
					worker_active_noun +
					' assigned.';
			}
		}
	}

	const dateClick = (event: any) => {
		const mydate = date;
		if (mydate) {
			stagingSelectedDate.update(() => {
				console.debug('Updating date', mydate);
				return mydate;
			});

			if ($stagingContext === RightPanelContext.edit) {
				const epoch_day = localDateToEpochDay(mydate);
				if (epoch_day) {
					const pertinent_ras = $stagingProcessedMembers?.rendered_assignables_by_date[epoch_day];

					if (pertinent_ras) {
						const selection = new Set<Selectable>();
						{
							const current_selection = $stagingSelection.selections.get(
								StagingSelectionMode.primary_selected
							);
							if (current_selection) {
								current_selection.forEach((s) => {
									selection.add(s);
								});
							}
						}

						let already_selected = true;
						for (const s of pertinent_ras.values()) {
							if (!selection.has(s)) {
								already_selected = false;
								break;
							}
						}

						if (!(event as MouseEvent).ctrlKey) {
							selection.clear();
						}

						if (already_selected) {
							pertinent_ras.forEach((s) => {
								selection.delete(s);
							});
						} else {
							pertinent_ras.forEach((s) => {
								selection.add(s);
							});
						}

						stagingSelection.setSelection(StagingSelectionMode.primary_selected, selection);
					}
				}
			}
		}
	};
</script>

<td>
	<div class="cont1 table_default_borders">
		{#if summary}
			<Wrapper>
				<div class="chip" style="background-color:{required_color}">
					<div class="chipcontents">
						{worker_range_string}
					</div>
				</div>
				<Tooltip>{required_worker_tooltip}</Tooltip>
			</Wrapper>
			<Wrapper>
				<div class="chip" style="background-color:{assigned_color}">
					<div class="chipcontents">
						{summary.worker_assigned}/{summary.worker_active}
					</div>
				</div>
				<Tooltip>{assigned_worker_tooltip}</Tooltip>
			</Wrapper>
		{/if}
		<div class="cont2">
			{#if date}
				<Button on:click={dateClick} variant="raised">
					<Label>{toDateString_ShortDisplay(date)}</Label>
				</Button>
			{/if}
		</div>
	</div>
</td>

<style>
	td {
		box-sizing: border-box;
		position: sticky;
		top: 25px;
		min-height: 100%;
		height: 35px;
		background: black;
		padding: 0px;
		margin: 0px;
		z-index: 1;
		border: 0px;
	}
	.chip {
		display: flex;
		align-items: center;
		justify-items: center;
		border-color: transparent;
		border-radius: 15px;
		padding: 10px;
	}
	.chipcontents {
		height: fit-content;
		width: fit-content;
		pointer-events: none;
		user-select: none;
	}
	.cont1 {
		display: flex;
		flex-direction: row;
		box-sizing: border-box;
		align-content: center;
		justify-content: space-between;
		height: 100%;
		min-height: 25px;
		padding: 2px;
		overflow-x: hidden;
	}
	.cont2 {
		display: flex;
		flex-direction: column;
		align-content: center;
		justify-content: center;
	}
</style>
