import {
	KeyboardArrowLeft,
	KeyboardArrowRight,
	KeyboardDoubleArrowLeft,
	KeyboardDoubleArrowRight
} from '@mui/icons-material';
import AddCircleOutlineIcon from '@mui/icons-material/AddCircleOutline';
import BlockIcon from '@mui/icons-material/Block';
import CloseIcon from '@mui/icons-material/Close';
import EditIcon from '@mui/icons-material/Edit';
import ImportExportIcon from '@mui/icons-material/ImportExport';
import {
	Box,
	Button,
	Chip,
	Dialog,
	DialogContent,
	DialogTitle,
	FormControl,
	Grid,
	IconButton,
	InputLabel,
	MenuItem,
	Select,
	SelectChangeEvent,
	TableRow,
	Typography
} from '@mui/material';
import { DatePicker, LocalizationProvider } from '@mui/x-date-pickers';
import { AdapterDateFns } from '@mui/x-date-pickers/AdapterDateFns';
import { DateValidationError } from '@mui/x-date-pickers/internals/hooks/validation/useDateValidation';
import { formatISO, parseISO } from 'date-fns';
import { ChangeEvent, default as React, FC, useEffect, useMemo } from 'react';
import { ASDisplayResponseData, getValidOrderedColumnIndices } from '../../ajax/commands_generic';
import { CSV_Builder, parseCSV } from '../../commons/CSV';
import { toDateString_ShortDisplay } from '../../commons/Dates';
import { ShowRawTableCopyDialog, ShowRawTableEditDialog } from '../../input/dialogs';
import { processDateChange, renderDatePickerInput } from '../../input/LinkedDatePicker';
import { WrappedHook } from '../../react/WrappedHook';
import { CreateTable, left_styles, StyledCell, topleft_styles, top_styles } from '../db/table';
import { Display_Main_Props } from '../display';

const date_column_key = 0;
const worker_column_key = 1;
const assignment_type_column_key = 2;
const responsibility_column_key = 3;

interface ResponsibilityChange {
	date: Date;
	rowkey: string;
	responsibility: number | undefined;
}

interface RawRow {
	[assignment_type_column_key]: number;
	[worker_column_key]: number;
	[date_column_key]: string;
	[responsibility_column_key]: number;
}

const right_border: React.CSSProperties = {
	borderRightStyle: 'solid',
	borderRightWidth: '1',
	borderRightColor: left_styles.borderColor
};

const delta_top_styles: React.CSSProperties = {
	...top_styles,
	...right_border,
	justifyContent: 'center'
};

const dateToISO = (date: Date) => {
	return formatISO(date, { representation: 'date' });
};

export const ResponsibilityDisplay: FC<Display_Main_Props> = (props: Display_Main_Props) => {
	console.debug('Prerendering ResponsibilityDisplay');
	const rawtabledata = props.response.update_data as ASDisplayResponseData;

	const current_date = new WrappedHook<Date>(null);

	const assignment_type_column = rawtabledata.cols[assignment_type_column_key];
	const worker_column = rawtabledata.cols[worker_column_key];

	const assignmenttypes_in_table = useMemo(
		() => getValidOrderedColumnIndices(assignment_type_column),
		[assignment_type_column]
	);
	const workers_in_table = useMemo(
		() => getValidOrderedColumnIndices(worker_column),
		[worker_column]
	);

	console.debug('Workers', workers_in_table);

	//const getColumnIndex=(assignment_type_id:number|string)=>{return assignmenttypes_in_table.by_member_id[assignment_type_id]+1;}
	//const getRowIndex=(worker_id:number|string)=>{return workers_in_table.by_member_id[worker_id];}

	const createHeaderCells = () => {
		const changed_topleft_styles = {} as React.CSSProperties;
		Object.assign(changed_topleft_styles, topleft_styles);
		changed_topleft_styles.width = 'max-content';
		let header_cells: React.ReactNode[] = [];
		{
			header_cells.push(<StyledCell key="First Column" align="center" style={topleft_styles} />);

			for (const order_index in assignmenttypes_in_table.by_order) {
				const assignment_type_id = assignmenttypes_in_table.by_order[order_index];

				const headercellkey = 'Header ' + assignment_type_id;

				header_cells.push(
					<StyledCell key={headercellkey} align="left" style={delta_top_styles}>
						<Grid container direction="row" justifyContent="flex-start" alignItems="center">
							<Grid item xs="auto">
								<Typography>
									{
										rawtabledata.cols[assignment_type_column_key].meta.labels.map[
											assignment_type_id
										]
									}
								</Typography>
							</Grid>
						</Grid>
					</StyledCell>
				);
			}
		}
		return header_cells;
	};

	const header_cells = useMemo(createHeaderCells, [assignmenttypes_in_table, rawtabledata.cols]);

	//Parse responsibilities
	let parsed_responsibilities: {
		[i: number | string]: { [i: number | string]: ResponsibilityChange | null };
	} = {};

	for (const worker_id of worker_column.meta.labels.order) {
		if (worker_column.meta.labels.disabled && !worker_column.meta.labels.disabled[worker_id]) {
			parsed_responsibilities[worker_id] = {} as {
				[i: number | string]: ResponsibilityChange | null;
			};
		}
	}

	//const dt1 = Date.now();

	const dates = [] as Date[];
	if (current_date.get()) {
		dates.push(current_date.get());
	}
	for (const rowkey in rawtabledata.rows) {
		const rowdat = rawtabledata.rows[rowkey];

		let parsed_row = parsed_responsibilities[rowdat[worker_column_key]];

		if (parsed_row) {
			const previous_change = parsed_row[rowdat[assignment_type_column_key]];

			const this_change = {} as ResponsibilityChange;
			this_change.rowkey = rowkey;
			this_change.date = parseISO(rowdat[date_column_key]);
			this_change.responsibility = rowdat[responsibility_column_key];

			if (
				this_change.date <= current_date.get() &&
				(!previous_change || this_change.date > previous_change.date)
			) {
				parsed_row[rowdat[assignment_type_column_key]] = this_change;
			}

			let add_date = true;
			for (const d of dates) {
				if (d.valueOf() == this_change.date.valueOf()) {
					add_date = false;
					break;
				}
			}
			if (add_date) {
				dates.push(this_change.date);
			}
		}
	}

	//const dt2 = Date.now();

	//Sort dates
	dates.sort((a: Date, b: Date) => {
		return a.valueOf() - b.valueOf();
	});

	//const dt3 = Date.now();

	//const dates = getDates();

	const last_date_in_db = dates[dates.length - 1];

	//On first render, set date to date of last change
	useEffect(() => {
		if (last_date_in_db) {
			current_date.set(last_date_in_db);
		} else {
			current_date.set(new Date());
		}
	}, []);

	const left_column_style = {} as React.CSSProperties;
	//Object.assign(left_column_style,left_styles);
	//left_column_style.width=undefined;
	const disabled_icon_color = 'disabled';

	//Create rows
	const responsibilities_this_date = {};

	const rows: React.ReactNode[] = [];
	for (const worker_order_index in workers_in_table.by_order) {
		const worker_id = workers_in_table.by_order[worker_order_index];
		const parsed_row = parsed_responsibilities[worker_id];
		const cells: React.ReactNode[] = [];
		//Left column
		cells.push(
			<StyledCell key={'lc' + worker_id} style={left_styles}>
				<Box display="flex" justifyContent="flex-end" alignContent="center">
					<Typography width="min-content">
						{rawtabledata.cols[worker_column_key].meta.labels.map[worker_id]}
					</Typography>
				</Box>
			</StyledCell>
		);
		if (parsed_row) {
			for (const assignable_order_index in assignmenttypes_in_table.by_order) {
				const assignable_id = assignmenttypes_in_table.by_order[assignable_order_index];
				const parsed_cell = parsed_row[assignable_id];
				let resp: React.ReactNode = null;
				let button: React.ReactNode = null;

				if (parsed_cell && current_date.get()) {
					if (parsed_cell.date <= current_date.get()) {
						console.debug('Parsed Cell', parsed_cell);

						let color: 'default' | 'success' | 'disabled';
						if (parsed_cell.date < current_date.get()) {
							color = 'default';
						} else {
							color = 'success';
						}

						if (responsibilities_this_date[worker_id] == undefined) {
							responsibilities_this_date[worker_id] = {};
						}

						if (parsed_cell.responsibility != null && parsed_cell.responsibility != undefined) {
							resp = <Chip color={color} label={parsed_cell.responsibility} />;
							responsibilities_this_date[worker_id][assignable_id] = parsed_cell.responsibility;
						} else {
							if (color == 'default') {
								color = disabled_icon_color;
							}
							resp = <BlockIcon color={color} />;
							responsibilities_this_date[worker_id][assignable_id] = '';
						}
					}

					if (parsed_cell.date.valueOf() == current_date.get().valueOf()) {
						const onClick = () => {
							props.app.showDialog(
								ShowRawTableEditDialog({
									display_props: props,
									row_key: parsed_cell.rowkey
								})
							);
						};
						button = (
							<IconButton onClick={onClick}>
								<EditIcon />
							</IconButton>
						);
					}
				}

				if (resp == null) {
					resp = <BlockIcon color={disabled_icon_color} />;
				}
				if (button == null) {
					const onClick = () => {
						const original_row: RawRow = {} as RawRow;
						original_row[assignment_type_column_key] = assignable_id;
						original_row[worker_column_key] = worker_id;
						original_row[date_column_key] = dateToISO(current_date.get());
						original_row[responsibility_column_key] = 1;
						ShowRawTableCopyDialog(props, original_row);
					};
					button = (
						<IconButton id={'add_' + worker_id + '_' + assignable_id} onClick={onClick}>
							<AddCircleOutlineIcon />
						</IconButton>
					);
				}

				cells.push(
					<StyledCell key={assignable_id} style={right_border}>
						<Grid
							container
							direction="row"
							justifyContent="space-evenly"
							alignItems="center"
							columns={{ xs: 2 }}
						>
							<Grid item xs={1} display="flex" justifyContent="center">
								{resp}
							</Grid>
							<Grid item xs={1} display="flex" justifyContent="center">
								{button}
							</Grid>
						</Grid>
					</StyledCell>
				);
			}
			rows.push(<TableRow key={'r' + worker_id}>{cells}</TableRow>);
		}
	}

	//const dt4 = Date.now();

	if (current_date.get() == null) {
		console.debug('Date is null. Returning null.');
		return null;
	}

	const date_menuitems = [] as React.ReactElement[];
	dates.forEach((d: Date) => {
		const dstr = toDateString_ShortDisplay(d);
		const vstr = d.toISOString();
		date_menuitems.push(
			<MenuItem key={dstr} value={vstr}>
				{dstr}
			</MenuItem>
		);
	});

	let previous_index = 0;
	let next_index = dates.length - 1;
	for (let n = 0; n < dates.length - 1; n++) {
		if (current_date.get() > dates[n]) {
			console.debug(current_date.get() + ' is later than ' + dates[n]);
			previous_index = n;
		} else {
			console.debug(current_date.get() + ' is not later than ' + dates[n]);
			break;
		}
	}
	for (let n = dates.length - 1; n >= 0; n--) {
		if (current_date.get() < dates[n]) {
			console.debug(current_date.get() + ' is earlier than ' + dates[n]);
			next_index = n;
		} else {
			console.debug(current_date.get() + ' is not earlier than ' + dates[n]);
			break;
		}
	}
	const previous_disabled = current_date.get() <= dates[0];
	const next_disabled = current_date.get() >= dates[dates.length - 1];
	console.debug('Next date is ', dates[next_index]);
	console.debug('Previous date is ', dates[previous_index]);

	const picker = (
		<FormControl id="picker" variant="standard">
			<InputLabel id="label">Date</InputLabel>
			<Select
				labelId="label"
				id="select"
				value={current_date.get().toISOString()}
				onChange={(event: SelectChangeEvent<string>, child: React.ReactNode) => {
					current_date.set(parseISO(event.target.value));
				}}
				label="Age"
			>
				{date_menuitems}
			</Select>
		</FormControl>
	);

	const datechange = (newdate: Date, keyboardInputValue?: string) => {
		const processed_date = processDateChange(newdate, keyboardInputValue);
		if (processed_date) {
			current_date.set(processed_date);
		}
	};

	const datepicker = (
		<LocalizationProvider dateAdapter={AdapterDateFns}>
			<DatePicker
				label={'Date'}
				value={current_date.get()}
				onChange={datechange}
				renderInput={renderDatePickerInput('datepickerinput')}
				//minDate={minDate}
				//maxDate={maxDate}
				inputFormat="MM/dd/yyyy"
				//onAccept={(d:Date)=>{current_date.set(d);}}
				disabled={false}
				onOpen={() => {
					console.debug('Open.');
				}}
				onClose={() => {
					console.debug('Close.');
				}}
				onError={(reason: DateValidationError, value: any) => {
					console.debug('Error', reason, value);
				}}
			/>
		</LocalizationProvider>
	);

	const switch_date_by_index = (index: number) => {
		console.debug(index, dates);
		if (dates[index]) {
			current_date.set(dates[index]);
		}
	};

	const download = () => {
		props.app.showDialog(null);
		const download_anchor = document.createElement('a');

		const output = new CSV_Builder();
		output.addStringField('Worker');

		for (const order_index in assignmenttypes_in_table.by_order) {
			const assignment_type_id = assignmenttypes_in_table.by_order[order_index];
			const assignment_name =
				rawtabledata.cols[assignment_type_column_key].meta.labels.map[assignment_type_id];
			output.addStringField(assignment_name);
		}

		for (const worker_order_index in workers_in_table.by_order) {
			output.newRow();
			const worker_id = workers_in_table.by_order[worker_order_index];
			const worker_name = rawtabledata.cols[worker_column_key].meta.labels.map[worker_id];
			output.addStringField(worker_name);
			for (const order_index in assignmenttypes_in_table.by_order) {
				const assignment_type_id = assignmenttypes_in_table.by_order[order_index];
				if (
					responsibilities_this_date[worker_id] != undefined &&
					responsibilities_this_date[worker_id][assignment_type_id] != undefined
				) {
					output.addNumericField(responsibilities_this_date[worker_id][assignment_type_id]);
				} else {
					output.addEmptyField();
				}
			}
		}

		download_anchor.href = 'data:text/csv;charset=utf-8,' + encodeURI(output.getResult());
		download_anchor.target = '_export';
		download_anchor.download =
			'worker_responsibilities_' +
			current_date.get().getFullYear() +
			'-' +
			current_date.get().getMonth() +
			'-' +
			current_date.get().getDate() +
			'.csv';
		download_anchor.click();
	};

	const handleupload = (e: ChangeEvent<HTMLInputElement>) => {
		props.app.showDialog(null);
		console.debug('Upload', e);
		const reader = new FileReader();
		reader.onload = (e) => {
			if (!e?.target?.result) {
				return;
			}
			const str = e.target.result as string;
			const parsed_csv = parseCSV(str);

			const assignment_col_map = {};
			const worker_row_map = {};

			for (let n = 1; n < parsed_csv[0].length; n++) {
				const val = parsed_csv[0][n];

				for (const order_index in assignmenttypes_in_table.by_order) {
					const assignment_type_id = assignmenttypes_in_table.by_order[order_index];
					const assignment_name =
						rawtabledata.cols[assignment_type_column_key].meta.labels.map[assignment_type_id];

					if (val === assignment_name) {
						assignment_col_map[n] = assignment_type_id;
						break;
					}
				}
			}

			for (let n = 1; n < parsed_csv.length; n++) {
				const val = parsed_csv[n][0];

				for (const worker_order_index in workers_in_table.by_order) {
					const worker_id = workers_in_table.by_order[worker_order_index];
					const worker_name = rawtabledata.cols[worker_column_key].meta.labels.map[worker_id];

					console.debug('Comparing', val, worker_name);
					if (val === worker_name) {
						worker_row_map[n] = worker_id;
						break;
					}
				}
			}

			console.debug('Maps', assignment_col_map, worker_row_map);

			const modified_rows = [];

			for (const row in worker_row_map) {
				const worker_id = worker_row_map[row];
				for (const col in assignment_col_map) {
					const assignment_type_id = assignment_col_map[col];
					const raw_val = parsed_csv[row][col];
					let processed_val = parseFloat(raw_val);
					if (raw_val === '' || raw_val === undefined) {
						processed_val = null;
					} else if (
						Number.isNaN(processed_val) ||
						!Number.isFinite(processed_val) ||
						processed_val === undefined
					) {
						const worker_name = rawtabledata.cols[worker_column_key].meta.labels.map[worker_id];
						const assignment_name =
							rawtabledata.cols[assignment_type_column_key].meta.labels.map[assignment_type_id];
						window.alert(
							'Malformatted entry value for ' +
								worker_name +
								'/' +
								assignment_name +
								': ' +
								raw_val +
								'(' +
								typeof raw_val +
								'). Aborting import.'
						);
						return;
					}

					const new_row = {} as RawRow;
					new_row[assignment_type_column_key] = assignment_type_id;
					new_row[worker_column_key] = worker_id;
					new_row[responsibility_column_key] = processed_val;
					new_row[date_column_key] = dateToISO(current_date.get());
					modified_rows.push(new_row);
				}
			}

			console.debug('Bulk modifying.', modified_rows);

			const handler = () => {
				props.app.sendDisplayRequest(props.request); //Just request full update.
			};

			props.app.bulkModify({ updates: modified_rows }, handler);
		};
		reader.readAsText(e.target.files[0]);
	};

	const deleteDate = () => {
		props.app.showDialog(null);

		const handler = () => {
			props.app.sendDisplayRequest(props.request); //Just reload.
		};

		props.app.bulkModify({ deletions: dateToISO(current_date.get()) }, handler);
	};

	const modal = (
		<Box>
			<Dialog
				open={true}
				fullWidth={false}
				maxWidth={false}
				disableEscapeKeyDown={false}
				fullScreen={false}
				aria-labelledby="form-dialog-title"
			>
				<DialogTitle
					key={'title'}
					display="flex"
					justifyContent="space-between"
					align-items="center"
				>
					{'Bulk Modification'}
					<IconButton
						onClick={() => {
							props.app.showDialog(null);
						}}
					>
						<CloseIcon />
					</IconButton>
				</DialogTitle>
				<DialogContent key={'content'}>
					<Box display="flex" flexDirection="column">
						<Button
							component="label"
							key="export"
							color="primary"
							variant="contained"
							aria-label="Export"
							onClick={download}
							style={{ marginBottom: '5px' }}
						>
							<Typography>Export</Typography>
						</Button>
						<Button
							component="label"
							key="import"
							color="primary"
							variant="contained"
							aria-label="Import"
							style={{ marginBottom: '5px' }}
						>
							<Typography>Import</Typography>
							<input type="file" id="upload_input" accept=".csv" hidden onChange={handleupload} />
						</Button>
						<Button
							component="label"
							key="delete"
							color="secondary"
							variant="contained"
							aria-label="Export"
							onClick={deleteDate}
							style={{ marginBottom: '5px' }}
						>
							<Typography>Delete This Date</Typography>
						</Button>
					</Box>
				</DialogContent>
			</Dialog>
		</Box>
	);

	const footer = (
		<Box
			height="min-content"
			width="100%"
			display="flex"
			flexDirection="row"
			justifyContent="space-around"
			alignContent="space-between"
		>
			<Box
				key="footer"
				display="flex"
				flexDirection="row"
				justifyContent="space-between"
				flexGrow="1"
				alignContent="space-between"
			>
				<Box
					key="picker_area"
					display="flex"
					flexDirection="row"
					flexGrow="1"
					justifyContent="center"
					alignContent="space-between"
				>
					<IconButton
						key="beg"
						color="primary"
						aria-label="Beginning of list markers"
						onClick={() => {
							switch_date_by_index(0);
						}}
						disabled={previous_disabled}
					>
						<KeyboardDoubleArrowLeft />
					</IconButton>
					<IconButton
						key="left"
						color="primary"
						aria-label="Back one"
						onClick={() => {
							switch_date_by_index(previous_index);
						}}
						disabled={previous_disabled}
					>
						<KeyboardArrowLeft />
					</IconButton>
					{picker}
					<IconButton
						key="right"
						color="primary"
						aria-label="Forward one"
						onClick={() => {
							switch_date_by_index(next_index);
						}}
						disabled={next_disabled}
					>
						<KeyboardArrowRight />
					</IconButton>
					<IconButton
						key="end"
						color="primary"
						aria-label="End of list markers"
						onClick={() => {
							switch_date_by_index(dates.length - 1);
						}}
						disabled={next_disabled}
					>
						<KeyboardDoubleArrowRight />
					</IconButton>
				</Box>
				<Box key="datepickerarea" display="flex" flexGrow="1">
					{datepicker}
				</Box>
			</Box>
			<Box
				key="bulkarea"
				display="flex"
				flexDirection="row"
				flexShrink="1"
				justifyContent="center"
				alignContent="space-between"
			>
				<IconButton
					key="display_bulk_menu"
					color="primary"
					aria-label="Export"
					onClick={() => {
						props.app.showDialog(modal);
					}}
				>
					<ImportExportIcon />
				</IconButton>
			</Box>
		</Box>
	);

	const boxstyle: React.CSSProperties = {
		width: '100%',
		height: '100%',
		display: 'flex',
		flexDirection: 'column'
	};
	const retval = (
		<Box key="responsibility_table_container" style={boxstyle}>
			<Box key="mainbox" width="100%" height="100%" overflow="scroll">
				<Box width="100%" height="100%">
					<CreateTable
						id="responsibility_table"
						table_style={{
							minWidth: (Object.keys(assignmenttypes_in_table.by_member_id).length + 1) * 150
						}}
						header_cells={header_cells}
						rows={rows}
						layout={'auto'}
						reset_pagination_side_effect_manager={null}
						enable_pagination={false}
					/>
				</Box>
			</Box>
			<Box key="footerbox">{footer}</Box>
		</Box>
	);

	//const dt5 = Date.now();

	console.debug('Returning responsibility.', header_cells, rows);
	//console.debug("Timing",dt2-dt1,dt3-dt2,dt4-dt3,dt5-dt4);

	return retval;
};
