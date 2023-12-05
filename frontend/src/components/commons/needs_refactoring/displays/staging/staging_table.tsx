import { Grid, Typography } from '@mui/material';
import { addWeeks } from 'date-fns';
import React, { FC, useMemo } from 'react';
import { WrappedHook } from '../../react/WrappedHook';
import { DayOfTheWeek } from './data_processing/processing01';
import { ProcessingResult02 } from './data_processing/processing02';
import { HeaderCell, left_column_width, table_item_height } from './staging';
import {
	CellContents,
	ListMarker,
	NavigatedDebouncedAutosizedStickyVirtualTable
} from './sticky_virtual_table';
import { days_per_week, SortedRenderedResult, Week } from './week_sorting';

interface StagingTableProps {
	preprocessed_data: ProcessingResult02;
	//update_data:ASStagingResponseMessage,
	//interactionHandler:InteractionHandler
}

const getHeaderCells = () => {
	const header_cells: React.ReactElement[] = [];
	header_cells.push(
		<HeaderCell>
			<CellContents key="pcol" alignContent="center">
				<Typography>P</Typography>
			</CellContents>
		</HeaderCell>
	);
	for (let n = 0; n < days_per_week; n++) {
		header_cells.push(
			<HeaderCell>
				<CellContents key={'dow' + n} alignContent="left">
					<Grid container direction="row" justifyContent="center" alignItems="center" width="100%">
						<Grid item xs="auto">
							<Typography>{DayOfTheWeek[n]}</Typography>
						</Grid>
					</Grid>
				</CellContents>
			</HeaderCell>
		);
	}
	return header_cells;
};

const getColumnWidths = (table_width: number) => {
	const column_widths = [] as number[];
	column_widths[0] = left_column_width;
	const width_remaining = Math.floor(table_width - column_widths[0]);
	for (let n = 0; n < days_per_week; n++) {
		column_widths[n + 1] = width_remaining / days_per_week;
	}
	return column_widths;
};

export const StagingTable: FC<StagingTableProps> = (props: StagingTableProps) => {
	console.debug('Rendering staging table!');

	const week_start = new WrappedHook<number>(0);
	const week_end = new WrappedHook<number>(props.preprocessed_data.weeks.length - 1);

	const getMoreRows = (needRowsAtBeginning: boolean) => {
		const delta = 4;
		const default_start = -1;
		const default_end = props.preprocessed_data.weeks.length;
		if (needRowsAtBeginning) {
			console.debug('More rows requested for the beginning.');
			let new_val = week_start.get() - delta;
			if (new_val > default_start) {
				new_val = default_start;
			}
			week_start.set(new_val);
			week_end.set(default_end);
		} else {
			console.debug('More rows requested for the end.');
			let new_val = week_end.get() + delta;
			if (new_val < default_end) {
				new_val = default_end;
			}
			week_end.set(new_val);
			week_start.set(default_start);
		}
		return delta;
	};

	//Displayed rows
	const memoized_rows = useMemo<SortedRenderedResult>(() => {
		const allrows: SortedRenderedResult = {} as SortedRenderedResult;
		allrows.row_cells = [];
		allrows.row_styles = [];
		allrows.week_indices = [] as ListMarker[];

		//const r1=new Date().getTime();
		for (let key = week_start.get(); key <= week_end.get(); key++) {
			const week = props.preprocessed_data.weeks[key];
			let result: SortedRenderedResult;
			if (week !== undefined) {
				result = week.render(key.toString());
			} else {
				const start_of_week: Date = addWeeks(props.preprocessed_data.first_sunday, key);
				result = Week.render_emptyweek(start_of_week);
			}

			for (let n = 0; n < result.week_indices.length; n++) {
				//Adjust indices for the larger array before concatenation
				result.week_indices[n].index += allrows.row_cells.length;
			}

			allrows.row_cells = allrows.row_cells.concat(result.row_cells);
			allrows.row_styles = allrows.row_styles.concat(result.row_styles);
			allrows.week_indices = allrows.week_indices.concat(result.week_indices);
		}

		return allrows;
	}, [
		week_start.get(),
		week_end.get(),
		props.preprocessed_data
		//display_props.response.update_data,
		//selectedDate
	]);

	const table = (
		<NavigatedDebouncedAutosizedStickyVirtualTable
			getColumnWidth={getColumnWidths}
			header_row={getHeaderCells()}
			content_rows={memoized_rows.row_cells}
			content_row_styles={memoized_rows.row_styles}
			itemHeight={table_item_height}
			getMoreRows={getMoreRows}
			list_markers={memoized_rows.week_indices}
		/>
	);

	return table;
};
