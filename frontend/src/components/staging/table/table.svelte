<script lang="ts">
	import Week from './week.svelte';

	import { addDays } from 'date-fns';
	import { localDateToEpochDay } from '../../commons/refactored/staging/data_processing/processing01';
	import type { ProcessingResult02 } from '../../commons/refactored/staging/data_processing/processing02';
	import { stagingSelectedDate } from '../stores';
	import './table.css';

	export let staging_data: ProcessingResult02;

	//Binding week elements for scrolling and manipulation
	const week_elements: { [epoch_day: number]: HTMLElement } = {};
	function getIndex(date: Date | undefined) {
		if (!date) {
			return -1;
		}
		const epoch_day = localDateToEpochDay(date);
		if (epoch_day) {
			return epoch_day;
		} else {
			return -1;
		}
	}

	//When selected date changes, scroll to that date
	$: {
		let sunday = $stagingSelectedDate;
		sunday = addDays(sunday, -sunday.getDay());
		const index = getIndex(sunday);
		const week_element = week_elements[index];
		if (week_element) {
			week_element.scrollIntoView(true);
		}
	}
</script>

<div class="container">
	<table>
		<tr class="header_row">
			<th class="header_corner"
				><div class="day_title table_default_borders table_header_background" /></th
			>
			<th class="header_day"
				><div class="day_title table_default_borders table_header_background">Sunday</div></th
			>
			<th class="header_day"
				><div class="day_title table_default_borders table_header_background">Monday</div></th
			>
			<th class="header_day"
				><div class="day_title table_default_borders table_header_background">Tuesday</div></th
			>
			<th class="header_day"
				><div class="day_title table_default_borders table_header_background">Wednesday</div></th
			>
			<th class="header_day"
				><div class="day_title table_default_borders table_header_background">Thursday</div></th
			>
			<th class="header_day"
				><div class="day_title table_default_borders table_header_background">Friday</div></th
			>
			<th class="header_day"
				><div class="day_title table_default_borders table_header_background">Saturday</div></th
			>
		</tr>
		{#each staging_data.weeks as week_instance (week_instance?.getSunday())}
			<Week
				bind:toprow={week_elements[getIndex(week_instance?.getSunday())]}
				week_data={week_instance}
			/>
		{/each}
	</table>
</div>

<style>
	div {
		box-sizing: border-box;
		height: 100%;
		flex-grow: 1;
		flex-shrink: 1;
		overflow-y: hidden;
	}
	.container {
		overflow-y: auto;
	}
	table {
		flex-grow: 1;
		display: table;
		table-layout: fixed;
		height: 100%;
		max-height: 100%;
		width: 100%;
		min-width: 100%;
		max-width: 100%;
		overflow-y: visible;
		overflow-x: hidden;
		border-collapse: collapse;
	}
	tr {
		box-sizing: border-box;
	}
	th {
		flex-grow: 1;
		position: sticky;
		top: 0px;
		height: 20px;
		background: black;
		padding: 0px;
		/*overflow-x:hidden;*/
	}
	.header_corner {
		box-sizing: border-box;
		width: 12px;
		height: 25px;
		z-index: 1;
	}
	.header_day {
		box-sizing: border-box;
		height: 25px;
		width: calc(100%-12px / 7);
		z-index: 1;
	}
	.day_title {
		display: block;
		box-sizing: border-box;
	}
</style>
