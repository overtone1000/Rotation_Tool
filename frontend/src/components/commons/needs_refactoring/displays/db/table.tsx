import {
	Box,
	styled,
	Table,
	TableBody,
	TableCell,
	TableContainer,
	TableFooter,
	TableHead,
	TablePagination,
	TableRow
} from '@mui/material';
import TablePaginationActions from '@mui/material/TablePagination/TablePaginationActions';
import React, { FC } from 'react';
import { SideEffectManager, WrappedHook } from '../../react/WrappedHook';
import { autoscheda_theme } from '../../theming/theme';

const StyledTable = styled(Table)({
	borderCollapse:
		'separate' /* Make sticky positioning work with borders, but still not working right... */,
	borderSpacing: 0,
	border: 0,
	padding: 0,
	margin: 0
});

export const StyledCell = styled(TableCell)({
	padding: 1,
	margin: 0
});

export const borderrow_height = 3;

const borderrow: React.CSSProperties = {
	backgroundColor: autoscheda_theme.palette.primary.light,
	position: 'sticky',
	top: 0,
	height: borderrow_height,
	borderStyle: 'none',
	zIndex: 3,
	lineHeight: 0,
	overflow: 'hidden',
	whiteSpace: 'nowrap'
};

const bordercell: React.CSSProperties = {
	padding: 0,
	margin: 0,
	border: 0
};

export const topleft_styles: React.CSSProperties = {
	backgroundColor: autoscheda_theme.palette.primary.dark,
	//color:autoscheda_theme.palette.common.black,
	position: 'sticky',
	top: 0,
	left: 0,
	zIndex: 3,
	borderTop: 0,
	borderBottom: 0,
	borderLeft: 0,
	borderRight: 3,
	borderColor: autoscheda_theme.palette.primary.light,
	borderStyle: 'solid',
	width: 100
};

export const top_styles: React.CSSProperties = {
	padding: 0 //Made this zero for virtualized row alignment...
	// width:100 //Don't do this here! with "fixed" tableLayout, this will override the effect of contents. Only do this in the calling functions if desired.
};

export const header_styles: React.CSSProperties = {
	backgroundColor: autoscheda_theme.palette.primary.dark,
	//color:autoscheda_theme.palette.common.black,
	position: 'sticky',
	top: 0,
	zIndex: 2,
	borderTop: 0,
	borderBottom: 0,
	borderLeft: 0,
	borderRight: 0,
	borderColor: autoscheda_theme.palette.primary.light,
	borderStyle: 'solid',
	height: 0 //this serves as the MINIMUM height for its members, so set to 0 so they can be changed
};

export const left_styles: React.CSSProperties = {
	backgroundColor: autoscheda_theme.palette.primary.dark,
	//color:autoscheda_theme.palette.common.black,
	position: 'sticky',
	left: 0,
	zIndex: 1,
	borderTop: 0,
	borderBottom: 1,
	borderLeft: 0,
	borderRight: 3,
	borderColor: autoscheda_theme.palette.primary.light,
	borderStyle: 'solid'
};

const body_styles: React.CSSProperties = {
	flexGrow: 1,
	flexShrink: 1,
	position: 'relative',
	zIndex: 0,
	minWidth: '100%',
	maxWidth: '100%',
	//height:"auto",
	overflow: 'auto'
};

/*
HOW TABLE SPACING WORKS
-table container width is set to 100% to fill the parent
-table width is set to 100% to also fill the parent (which is table container)
-table layout can be set to 
  fixed: which causes a column width to be constrained to the width of the TOP CELL IN EACH COLUMN IF SPECIFIED
  auto: which causes the column width to depend on the entire table contents

Fixed width works better for bigger tables (renders faster, looks more uniform).
Specify the topleft style to have a width of 100px. As long as there are other columns with width that will change based on content, the table will expand OTHER columns to make it 100% wide
For keyed tables (like the schedule), specify all the top column widths in the calling function to make the table look nice with even column widths
For raw tables, use auto.
Simple tables could probably use some improvement in this regard.
*/

export interface TableProps {
	id?: string;
	header_cells: React.ReactNode[];
	rows?: React.ReactNode[];
	virtual_rows?: React.ReactNode;
	layout: 'auto' | 'fixed';
	reset_pagination_side_effect_manager: SideEffectManager<any> | null;
	enable_pagination: boolean;
	custom_pagination?: React.ReactNode;
	table_style?: React.CSSProperties;
}

export const CreateTable: FC<TableProps> = (props: TableProps) => {
	let initial_rowsperpage = 50;
	if (props.rows === undefined || props.rows.length < 50 || !props.enable_pagination) {
		initial_rowsperpage = -1;
	}

	const page = new WrappedHook<number>(0);
	const rowsPerPage = new WrappedHook<number>(initial_rowsperpage);
	const rowsPerPageOptions = [5, 10, 25, 50, 100, { label: 'All', value: -1 }];

	const handleChangePage: (
		event: React.MouseEvent<HTMLButtonElement> | null,
		newPage: number
	) => void = (event: React.MouseEvent<HTMLButtonElement> | null, newPage: number) => {
		page.set(newPage);
	};

	const handleChangeRowsPerPage = (
		event: React.ChangeEvent<HTMLInputElement | HTMLTextAreaElement>
	) => {
		rowsPerPage.set(parseInt(event.target.value, 10));
		page.set(0);
	};

	if (props.enable_pagination) {
		if (
			props.reset_pagination_side_effect_manager !== null &&
			props.reset_pagination_side_effect_manager !== undefined
		) {
			props.reset_pagination_side_effect_manager.addSideEffect(
				'Table Pagination Reset',
				(e: any) => {
					handleChangePage(null, 0);
				}
			);
		} else {
			console.debug('Pagination side effect not set.', props);
		}
	}

	let displayed_rows: React.ReactNode[] | React.ReactNode;
	if (props.rows !== undefined) {
		if (rowsPerPage.get() > 0) {
			const start = page.get() * rowsPerPage.get();
			const end = start + rowsPerPage.get();
			displayed_rows = props.rows.slice(start, end);
		} else {
			displayed_rows = props.rows;
		}
	} else if (props.virtual_rows !== undefined) {
		displayed_rows = props.virtual_rows;
	}

	let table_head: React.ReactElement = null;
	if (props.header_cells !== undefined && props.header_cells !== null) {
		table_head = (
			<TableHead style={header_styles}>
				<TableRow>{props.header_cells}</TableRow>
				<TableRow style={borderrow}>
					<TableCell colSpan={props.header_cells.length} style={bordercell}></TableCell>
					{/*border*/}
				</TableRow>
			</TableHead>
		);
	}

	let pagination_element = null;
	if (props.enable_pagination) {
		pagination_element = (
			<Box height="max-content" flex="0 0 content" borderTop="1px solid white">
				<TableContainer>
					<Table>
						<TableFooter>
							<TableRow>
								<TablePagination
									rowsPerPageOptions={rowsPerPageOptions}
									colSpan={3}
									count={props.rows.length}
									rowsPerPage={rowsPerPage.get()}
									page={page.get()}
									SelectProps={{
										inputProps: { 'aria-label': 'rows per page' },
										native: true
									}}
									onPageChange={handleChangePage}
									onRowsPerPageChange={handleChangeRowsPerPage}
									ActionsComponent={TablePaginationActions}
								/>
							</TableRow>
						</TableFooter>
					</Table>
				</TableContainer>
			</Box>
		);
	} else if (props.custom_pagination !== undefined) {
		pagination_element = props.custom_pagination;
	}

	const table_conatiner_style: React.CSSProperties = {
		width: '100',
		height: '100%',
		overflow: 'auto'
	}; //width:"max-content",
	const table_style: React.CSSProperties = { tableLayout: props.layout };
	Object.assign(table_style, props.table_style);

	return (
		<Box height="100%" width="100%" display="flex" flexDirection="column">
			<Box minHeight="0px" maxHeight="100%" flex="1 1 0px">
				<TableContainer id="tablecontainer" component={Box} style={table_conatiner_style}>
					<StyledTable id={props.id} aria-label="table" style={table_style}>
						{table_head}
						<TableBody id="tablebody" key="tablebody" style={body_styles}>
							{displayed_rows}
						</TableBody>
					</StyledTable>
				</TableContainer>
			</Box>
			{pagination_element}
		</Box>
	);
};
