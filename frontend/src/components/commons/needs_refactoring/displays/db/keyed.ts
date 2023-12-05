import { autoscheda_theme, sizing } from '../../theming/theme';
import { Display_Main_Props } from '../display';
import { CreateTable, StyledCell, left_styles, top_styles, topleft_styles } from './table';

interface KeyedTable_Col {
	name: string;
}

interface KeyedTable_Meta {}

export interface KeyedTable_Data {
	cols: { [key: string]: KeyedTable_Col };
	membermap: {};
	meta: KeyedTable_Meta;
	rows: { [key: string]: KeyedTable_Row };
}

interface KeyedTable_Row {
	features: { color: RowBGColor };
	values: {};
}

enum RowBGColor {
	Default = 3,
	Weekend = 4
}

export const KeyedTable: FC<Display_Main_Props> = (props: Display_Main_Props) => {
	console.debug('Rendering keyed table.');
	const keyedtabledata = props.response.update_data as KeyedTable_Data;

	let header_cells: React.ReactNode[] = [];
	header_cells.push(
		<StyledCell key="First Column" align="left" style={topleft_styles}></StyledCell>
	);

	let delta_top_style: React.CSSProperties = new Object();
	Object.assign(delta_top_style, top_styles);
	delta_top_style.width = 100;

	const column_filters = props.display_mod.column_filters;
	let shown_cols: string[];
	if (column_filters === null || column_filters === undefined) {
		shown_cols = Object.keys(keyedtabledata.cols);
	} else {
		shown_cols = column_filters;
	}

	for (const colkey of shown_cols) {
		const col = keyedtabledata.cols[colkey] as KeyedTable_Col;
		const headercellkey = 'Header ' + colkey;
		header_cells.push(
			<StyledCell key={headercellkey} align="left" style={delta_top_style}>
				{col.name}
			</StyledCell>
		);
	}

	let rows: React.ReactNode[] = [];

	for (const rowkey in keyedtabledata.rows) {
		const rowdat = keyedtabledata.rows[rowkey];
		let cells: React.ReactNode[] = [];
		cells.push(
			<StyledCell key={rowkey} style={left_styles}>
				{rowkey}
			</StyledCell>
		);
		for (const colkey of shown_cols) {
			//Need to iterate by colkey in case something's missing
			const celldat = rowdat.values[colkey];
			let cellval;
			if (Array.isArray(celldat) && celldat.length > 1) {
				const assignees = [];
				for (const index in celldat) {
					assignees.push(<Box>{keyedtabledata.membermap[celldat[index]]}</Box>);
				}
				const tooltip_contents = (
					<React.Fragment>
						<Box style={{ display: 'flex', flexDirection: 'column' }}>{assignees}</Box>
					</React.Fragment>
				);

				cellval = (
					<Tooltip title={tooltip_contents} arrow>
						<Chip style={{ fontSize: 10 }} label={celldat.length + ' members'} variant="outlined" />
					</Tooltip>
				);
			} else {
				cellval = keyedtabledata.membermap[celldat];
			}
			const cellelementkey = 'Cell ' + rowkey + '_' + colkey;
			cells.push(<StyledCell key={cellelementkey}>{cellval}</StyledCell>);
		}
		const elementkey = 'Row ' + rowkey;

		let color: string;
		switch (rowdat.features.color) {
			case RowBGColor.Weekend:
				color = autoscheda_theme.palette.background.default;
				break;
			case RowBGColor.Default:
			default:
				color = autoscheda_theme.palette.background.paper;
				break;
		}
		rows.push(
			<TableRow
				key={elementkey}
				style={{ height: sizing.keyed_row_height, backgroundColor: color }}
			>
				{cells}
			</TableRow>
		);
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

	console.debug('Finished keyed table render.');
	return retval;
};
