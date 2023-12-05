<script lang="ts">
	import type { Week, WeekRenderedResult } from '../../commons/refactored/staging/week_sorting';
	import DateCell from './date_cell.svelte';
	import AssignableCell from './rendered_assignable.svelte';
	import './table.css';

	export let week_data: Week | undefined;
	export let toprow: HTMLElement;

	let rendered_week: WeekRenderedResult | undefined = undefined;
	const columns = [0, 1, 2, 3, 4, 5, 6];
	let rows: number[] = [];
	let priority_toprows: {
		[row: number]: {
			rowspan: number;
			priority: number;
		};
	} = {};
	let type_toprows: number[] = [];
	if (week_data) {
		rendered_week = week_data.render();
		//console.debug(rendered_week);
		for (let n = 0; n < rendered_week.row_count; n++) {
			rows.push(n);
			//console.debug(rendered_week.cells[n]);
		}
		for (const priority_span_index in rendered_week.priority_row_spans) {
			const priority_span = rendered_week.priority_row_spans[priority_span_index];
			priority_toprows[priority_span.start] = {
				rowspan: priority_span.end - priority_span.start + 1,
				priority: parseInt(priority_span_index)
			};
		}
		for (const type_index in rendered_week.type_row_spans) {
			const type_span = rendered_week.type_row_spans[type_index];
			type_toprows.push(type_span.start);
		}
	}
	//<td class="table_header_background"></td>
</script>

{#if rendered_week != undefined}
	<tr bind:this={toprow}>
		<DateCell date={null} />
		{#each rendered_week.dates as date}
			<DateCell {date} />
		{/each}
	</tr>
	{#each rows as row}
		<tr>
			{#if priority_toprows[row]}
				<td class="table_header_background" rowspan={priority_toprows[row].rowspan}>
					<div class="priority">
						{priority_toprows[row].priority}
					</div>
				</td>
			{/if}
			{#each columns as col}
				<AssignableCell
					assignable={rendered_week.cells[row][col]}
					priority_top={priority_toprows[row] !== undefined}
					type_top={type_toprows.includes(row)}
				/>
			{/each}
		</tr>
	{/each}
{:else}
	<div>Rendering...</div>
{/if}

<style>
	td {
		box-sizing: border-box;
		border: 1px;
		border-style: solid;
		border-color: white;
	}
	.priority {
		display: block;
		width: 100%;
		height: 100%;
		position: sticky;
		top: 10%;
		bottom: 10%;
	}
</style>
