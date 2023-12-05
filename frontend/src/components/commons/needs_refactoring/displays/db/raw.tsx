import { AddCircle, Edit, FileCopy } from '@mui/icons-material';
import { Grid, IconButton, TableRow, Typography } from '@mui/material';
import React, { FC } from 'react';
import { ASDisplayResponseData, buildFilter, Filter } from '../../ajax/commands';
import { toDisplayNode } from '../../data_processing/data_types';
import {
	RawEditDialogProps,
	ShowRawTableCopyDialog,
	ShowRawTableEditDialog,
	ShowRawTableNewDialog
} from '../../input/dialogs';
import { WrappedHook } from '../../react/WrappedHook';
import { Display_Main_Props } from '../display';
import { DisplaySync } from '../DisplaySync';
import { CreateTable, left_styles, StyledCell, topleft_styles, top_styles } from './table';

export const RawTable: FC<Display_Main_Props> = (props: Display_Main_Props) => {
	console.debug('Prerendering raw table components.', props);
	const rawtabledata = props.response.update_data as ASDisplayResponseData;
	const syncer = new DisplaySync();

	//Determine shown columns
	const showcolkeys: string[] = [] as string[];
	{
		for (const colkey in rawtabledata.cols) {
			const col = rawtabledata.cols[colkey];
			if (!col.hidden) {
				showcolkeys.push(colkey);
			}
		}
	}
	console.debug('showcolkeys', showcolkeys);

	//Create rows
	let rows: { [rowkey: string]: React.ReactNode } = {};
	{
		for (const rowkey in rawtabledata.rows) {
			const rowdat = rawtabledata.rows[rowkey];

			const dialogprops: RawEditDialogProps = {
				display_props: props,
				row_key: rowkey
			};

			let delta_left_style: React.CSSProperties = new Object();
			Object.assign(delta_left_style, left_styles);
			delta_left_style.width = 'min-content';

			let cells: React.ReactNode[] = [];
			cells.push(
				<StyledCell key={rowkey} style={delta_left_style} size="small">
					<Grid
						style={{ width: 'max-content' }}
						container
						direction="row"
						justifyContent="space-around"
						alignItems="center"
					>
						<Grid item xs={6} style={{ width: 'max-content' }}>
							<IconButton
								name={'edit_' + rowkey}
								aria-label="Edit"
								onClick={(evt) => {
									props.app.showDialog(ShowRawTableEditDialog(dialogprops));
								}}
							>
								<Edit />
							</IconButton>
						</Grid>
						<Grid item xs={6} style={{ width: 'max-content' }}>
							<IconButton
								name={'copy_' + rowkey}
								aria-label="Copy"
								onClick={(evt) => {
									ShowRawTableCopyDialog(props, rowdat);
								}}
							>
								<FileCopy />
							</IconButton>
						</Grid>
					</Grid>
				</StyledCell>
			);
			for (const colkey of showcolkeys) {
				//Need to iterate by colkey in case something's missing
				//const celldat = rowdat[colkey];
				//const col = rawtabledata.cols[colkey];

				const celldisplaynode = toDisplayNode(rawtabledata, colkey, rowkey, syncer);
				const cellelementkey = 'Cell ' + rowkey + '_' + colkey;
				cells.push(
					<StyledCell style={celldisplaynode.style} key={cellelementkey}>
						{celldisplaynode.element}
					</StyledCell>
				);
			}
			const elementkey = 'Row ' + rowkey;
			rows[rowkey] = <TableRow key={elementkey}>{cells}</TableRow>;
		}
	}

	console.debug('Finished raw table prerender.', rows);
	return <RawTableComposed display_props={props} all_rows={rows} showcolkeys={showcolkeys} />;
};

interface RawTableComposedProps {
	display_props: Display_Main_Props;
	all_rows: { [rowkey: string]: React.ReactNode };
	showcolkeys: string[];
}

const RawTableComposed: FC<RawTableComposedProps> = (props: RawTableComposedProps) => {
	const rawtabledata = props.display_props.response.update_data as ASDisplayResponseData;
	console.debug('Rendering raw table.');

	const start_time = Date.now();

	const filters = new WrappedHook<Filter>({});

	let header_cells: React.ReactNode[] = [];
	{
		header_cells.push(
			<StyledCell key="First Column" align="center" style={topleft_styles}>
				<IconButton
					name="new_button"
					onClick={(evt) => {
						ShowRawTableNewDialog(props.display_props);
					}}
				>
					<AddCircle></AddCircle>
				</IconButton>
			</StyledCell>
		);

		for (const colkey of props.showcolkeys) {
			const col = rawtabledata.cols[colkey];

			//console.debug("Adding column ",col);

			const headercellkey = 'Header ' + colkey;

			let filter: React.ReactNode[] = buildFilter(colkey, rawtabledata, filters);

			header_cells.push(
				<StyledCell key={headercellkey} align="left" style={top_styles}>
					<Grid container direction="row" justifyContent="flex-start" alignItems="center">
						<Grid item xs="auto">
							<Typography>{col.name}</Typography>
						</Grid>
						{filter}
					</Grid>
				</StyledCell>
			);
		}
	}

	//Apply row filters
	let visible_rows: { [rowkey: string]: React.ReactNode } = {};
	//Object.assign(visible_rows,props.all_rows);
	const current_filters = filters.get();
	for (const rowkey in rawtabledata.rows) {
		const rowdat = rawtabledata.rows[rowkey];
		let shown = true;
		for (const colkey in current_filters) {
			const rowval = rowdat[colkey];
			const shownvals = current_filters[colkey];
			console.debug('Checking row.', rowval, shownvals);
			if (!shownvals.includes(rowval)) {
				//visible_rows[rowkey]=null;
				//console.debug("Val is not shown. Hiding row.",shownvals,rowval);
				shown = false;
				break;
			}
		}
		if (shown) {
			visible_rows[rowkey] = props.all_rows[rowkey];
		}
	}

	const retval = (
		<CreateTable
			header_cells={header_cells}
			rows={Object.values(visible_rows)}
			layout={'auto'}
			reset_pagination_side_effect_manager={filters.side_effects}
			enable_pagination={true}
		/>
	);

	const end_time = Date.now();
	const rowcount = Object.keys(visible_rows).length;
	const duration = end_time - start_time;
	const msperrow = (rowcount / duration) * 1000;
	console.debug(
		'Finished raw table render of ' +
			Object.keys(visible_rows).length +
			' rows in ' +
			duration +
			' ms (' +
			msperrow +
			' rows/s)'
	);
	return retval;
};
