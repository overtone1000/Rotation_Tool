import { TableRow } from '@mui/material';
import React, { FC } from 'react';
import { Display_Main_Props } from '../display';
import { CreateTable, StyledCell, top_styles } from './table';

interface SimpleTable_Data {
	headers: {};
	rows: {};
}

export const SimpleTable: FC<Display_Main_Props> = (props: Display_Main_Props) => {
	const simpletabledata = props.response.update_data as SimpleTable_Data;

	let header_cells: React.ReactNode[] = [];

	let delta_top_style: React.CSSProperties = new Object();
	Object.assign(delta_top_style, top_styles);
	delta_top_style.width = 100;

	const column_filters = props.display_mod.column_filters;
	console.debug('Column filters are ', column_filters);
	let headers_to_show: string[];
	if (column_filters === undefined || column_filters === null) {
		headers_to_show = Object.keys(simpletabledata.headers);
	} else {
		headers_to_show = [Object.keys(simpletabledata.headers)[0]].concat(column_filters);
	}
	console.debug('Headers to show are ', headers_to_show);

	for (const headerkey of headers_to_show) {
		header_cells.push(
			<StyledCell key={'HeaderElement ' + headerkey} align="left" style={delta_top_style}>
				{simpletabledata.headers[headerkey]}
			</StyledCell>
		);
	}

	let rows: React.ReactNode[] = [];
	for (const rowkey in simpletabledata.rows) {
		let cells: React.ReactNode[] = [];
		for (const cellkey of headers_to_show) {
			const cellelementkey = 'Cell ' + rowkey + '_' + cellkey;
			cells.push(
				<StyledCell key={cellelementkey}>{simpletabledata.rows[rowkey][cellkey]}</StyledCell>
			);
		}
		const elementkey = 'Row ' + rowkey;
		rows.push(<TableRow key={elementkey}>{cells}</TableRow>);
	}

	const retval = (
		<CreateTable
			header_cells={header_cells}
			rows={rows}
			layout={'fixed'}
			reset_pagination_side_effect_manager={props.app.main_display_modification.side_effects}
			enable_pagination={true}
		/>
	);

	return retval;
};
