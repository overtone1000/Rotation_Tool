import {
	KeyboardArrowLeft,
	KeyboardArrowRight,
	KeyboardDoubleArrowLeft,
	KeyboardDoubleArrowRight
} from '@mui/icons-material';
import {
	Box,
	FormControl,
	IconButton,
	InputLabel,
	MenuItem,
	Select,
	SelectChangeEvent,
	styled
} from '@mui/material';
import React, { FC, FunctionComponentElement, useMemo, useRef } from 'react';
import debounceRender from 'react-debounce-render';
import AutoSizer from 'react-virtualized-auto-sizer';
import { FixedSizeList } from 'react-window';
import { WrappedHook } from '../../react/WrappedHook';

export const CellContents = styled(Box)({
	width: '100%',
	height: '100%',
	borders: 0,
	margins: 0,
	padding: 0,
	display: 'flex',
	justifyContent: 'center',
	alignContent: 'center',
	flexShrink: '1',
	overflow: 'hidden'
});

export interface ListMarker {
	index: number;
	label: string;
}

interface ExternalProps1 {
	header_row: React.ReactElement[]; //external
	content_rows: React.ReactElement[][]; //external
	content_row_styles: React.CSSProperties[]; //external
	itemHeight: number; //external
}

interface InternalProps1 {
	initialScrollTop: number; //internal
	onItemsRendered: (newState: ScrollState) => void; //internal
}

interface StickyVirtualTableProps extends InternalProps1, ExternalProps1 {
	column_widths: number[]; //internal
	height: number; //internal
	width: number; //internal
}

interface ExternalProps2 {
	getColumnWidth: (total_table_width: number) => number[]; //external
}

type DebouncedAutosizedStickyVirtualTableProps = ExternalProps1 & InternalProps1 & ExternalProps2;

export interface NavigatedDebouncedAutosizedStickyVirtualTableProps
	extends ExternalProps1,
		ExternalProps2 {
	getMoreRows: (needRowsAtBeginning: boolean) => number; //external
	list_markers: ListMarker[]; //external
}

const forced_row_style: React.CSSProperties = {
	display: 'flex',
	flexDirection: 'row',
	width: '100%',
	height: '100%'
};

const formatRow = (
	row: React.ReactElement[],
	column_widths: number[],
	row_style: React.CSSProperties
) => {
	const final_row = [] as React.ReactElement[];
	for (let n = 0; n < row.length; n++) {
		final_row.push(
			<div
				key={n}
				style={{
					display: 'flex',
					width: column_widths[n],
					minWidth: column_widths[n],
					maxWidth: column_widths[n],
					flexShrink: 1,
					overflow: 'hidden'
				}}
			>
				{row[n]}
			</div>
		);
	}

	const final_row_style = {} as React.CSSProperties;
	Object.assign(final_row_style, row_style);
	Object.assign(final_row_style, forced_row_style);
	return <div style={final_row_style}>{final_row}</div>;
};

export interface ScrollState {
	overscanStartIndex: number;
	overscanStopIndex: number;
	visibleStartIndex: number;
	visibleStopIndex: number;
}

interface ScrollChange {
	scrollDirection: 'forward' | 'backward';
	scrollOffset: number;
	scrollUpdateWasRequested: boolean;
}

const StickyVirtualTable: FC<StickyVirtualTableProps> = (svt_props: StickyVirtualTableProps) => {
	const RawRow = (row_props: { index: number; style: React.CSSProperties }) => {
		return (
			<div key={row_props.index} style={row_props.style}>
				{formatRow(
					svt_props.content_rows[row_props.index],
					svt_props.column_widths,
					svt_props.content_row_styles[row_props.index]
				)}
			</div>
		);
	};

	//Memoization is very memory expensive! In a test, it increased utilization 5x! Pass on this.
	const MemoRow = React.memo(RawRow, (prevProps, nextProps) => {
		return prevProps.index == nextProps.index && prevProps.style == nextProps.style;
	});

	const sbg_header: React.CSSProperties = {
		scrollbarGutter: 'stable',
		overflow: 'auto'
	};

	const sbg_content: React.CSSProperties = {
		overflowX: 'hidden',
		overflowY: 'hidden'
	};

	const header_height = svt_props.itemHeight;
	const body_height = svt_props.height - header_height;

	const onScroll = (scrollChange: ScrollChange) => {
		//console.debug("onScroll",scrollChange);
	};

	return (
		<div style={{ width: svt_props.width, tableLayout: 'fixed' }}>
			<div style={sbg_header}>
				{formatRow(svt_props.header_row, svt_props.column_widths, {} as React.CSSProperties)}
			</div>
			<div style={sbg_content}>
				<FixedSizeList
					height={body_height}
					itemCount={svt_props.content_rows.length}
					itemSize={svt_props.itemHeight}
					width={svt_props.width}
					initialScrollOffset={svt_props.initialScrollTop}
					onItemsRendered={svt_props.onItemsRendered}
					onScroll={onScroll}
				>
					{RawRow}
				</FixedSizeList>
			</div>
		</div>
	);
};

const DebouncedAutosizedStickyVirtualTable: FC<DebouncedAutosizedStickyVirtualTableProps> = (
	dastvt_props: DebouncedAutosizedStickyVirtualTableProps
) => {
	const create_grid = (props: { height: number; width: number }) => {
		const svt_props: StickyVirtualTableProps = {
			column_widths: dastvt_props.getColumnWidth(props.width),
			height: props.height,
			width: props.width,
			header_row: dastvt_props.header_row,
			content_rows: dastvt_props.content_rows,
			content_row_styles: dastvt_props.content_row_styles,
			itemHeight: dastvt_props.itemHeight,
			initialScrollTop: dastvt_props.initialScrollTop,
			onItemsRendered: dastvt_props.onItemsRendered
		};

		const new_grid = React.createElement(StickyVirtualTable, svt_props);

		return new_grid;
	};

	const DebouncedAutosizer = debounceRender<{
		children: ({
			height,
			width
		}: {
			height: any;
			width: any;
		}) => FunctionComponentElement<StickyVirtualTableProps>;
	}>(AutoSizer, 100, { leading: true, trailing: false });

	let autosized = (
		<DebouncedAutosizer>
			{({ height, width }) => {
				return create_grid({ height: height, width: width });
			}}
		</DebouncedAutosizer>
	);

	return (
		<Box flexGrow="1" flexShrink="1">
			{autosized}
		</Box>
	);
};

export const NavigatedDebouncedAutosizedStickyVirtualTable: FC<
	NavigatedDebouncedAutosizedStickyVirtualTableProps
> = (ndastvt_props: NavigatedDebouncedAutosizedStickyVirtualTableProps) => {
	const focused_list_item = new WrappedHook<number>(0);

	const rerender_table_switch = useRef<boolean>(false);
	const rerenderTable = () => {
		rerender_table_switch.current = !rerender_table_switch.current;
	};

	const top_row = useRef<number>(0);

	const initialScrollTop = ndastvt_props.itemHeight * top_row.current;

	const onItemsRendered = (newState: ScrollState) => {
		//console.debug("onItemsRendered",newState,top_row.current);

		let requesting: boolean = false;
		if (top_row.current !== newState.visibleStartIndex) {
			//Only change top_row to this new state if there isn't a new forced top row or if the new state adheres to the new forced top row
			//console.debug("Setting new visible start index to ",newState.visibleStartIndex," was ",top_row.current);
			top_row.current = newState.visibleStartIndex;
			if (newState.visibleStartIndex == 0) {
				const delta = ndastvt_props.getMoreRows(true);
				top_row.current += delta;
				requesting = true;
			}
			if (newState.visibleStopIndex >= ndastvt_props.content_rows.length - 1) {
				const delta = ndastvt_props.getMoreRows(false);
				requesting = true;
			}
		}

		//Makes sure the grid is big enough
		//if(newState.overscanStartIndex>=newState.visibleStartIndex)
		//{
		//    const delta=ndastvt_props.getMoreRows(true);
		//}
		if (newState.overscanStopIndex <= newState.visibleStopIndex && !requesting) {
			const delta = ndastvt_props.getMoreRows(false);
		}

		let n = 0;
		for (n = 0; n < ndastvt_props.list_markers.length - 1; n++) {
			if (
				newState.visibleStartIndex >= ndastvt_props.list_markers[n].index &&
				newState.visibleStartIndex < ndastvt_props.list_markers[n + 1].index
			) {
				break;
			}
		}
		focused_list_item.set(n);
	};

	const setListItem = (list_item: number) => {
		console.debug('Setting list item', list_item);
		top_row.current = ndastvt_props.list_markers[list_item].index;
		focused_list_item.set(list_item);
		rerenderTable();
	};

	const pickListItem = (event: SelectChangeEvent<number>, child: React.ReactNode) => {
		console.debug('Pick list item');
		const list_item = event.target.value as number;
		console.debug(event, child);
		setListItem(list_item);
	};

	const week_items = [] as React.ReactElement[];
	for (let n = 0; n < ndastvt_props.list_markers.length; n++) {
		week_items.push(
			<MenuItem key={n} value={n}>
				{ndastvt_props.list_markers[n].label}
			</MenuItem>
		);
	}

	const week_picker = (
		<FormControl id="picker" variant="standard">
			<InputLabel id="label">Week</InputLabel>
			<Select
				labelId="label"
				id="select"
				value={focused_list_item.get()}
				onChange={pickListItem}
				label="Age"
			>
				{week_items}
			</Select>
		</FormControl>
	);

	//Have to memoize so that the component doesn't rerender when scrolling past list items (i.e when  focused_list_item is changed).
	const memoized_table = useMemo<React.ReactElement>(() => {
		const t1 = new Date().getTime();
		const retval = (
			<DebouncedAutosizedStickyVirtualTable
				header_row={ndastvt_props.header_row}
				content_rows={ndastvt_props.content_rows}
				content_row_styles={ndastvt_props.content_row_styles}
				itemHeight={ndastvt_props.itemHeight}
				getColumnWidth={ndastvt_props.getColumnWidth}
				onItemsRendered={onItemsRendered}
				initialScrollTop={initialScrollTop}
			/>
		);
		const t2 = new Date().getTime();
		console.debug(
			'DebouncedAutosizedStickyVirtualTable render finished in ' + (t2 - t1).toString() + ' ms.'
		);
		return retval;
	}, [
		ndastvt_props.header_row,
		ndastvt_props.content_rows,
		ndastvt_props.content_row_styles,
		ndastvt_props.itemHeight,
		ndastvt_props.getColumnWidth,
		rerender_table_switch.current
	]);

	return (
		<Box
			style={{ overflowY: 'scroll' }}
			display="flex"
			flexDirection="column"
			height="100%"
			flexGrow="1"
		>
			{memoized_table}
			<Box
				height="min-content"
				width="100%"
				display="flex"
				flexDirection="row"
				justifyContent="center"
				alignContent="space-between"
			>
				<IconButton
					key="beg"
					color="primary"
					aria-label="Beginning of list markers"
					onClick={() => {
						setListItem(0);
					}}
				>
					<KeyboardDoubleArrowLeft />
				</IconButton>
				<IconButton
					key="left"
					color="primary"
					aria-label="Back one"
					onClick={() => {
						setListItem(focused_list_item.get() - 1);
					}}
				>
					<KeyboardArrowLeft />
				</IconButton>
				{week_picker}
				<IconButton
					key="right"
					color="primary"
					aria-label="Forward one"
					onClick={() => {
						setListItem(focused_list_item.get() + 1);
					}}
				>
					<KeyboardArrowRight />
				</IconButton>
				<IconButton
					key="end"
					color="primary"
					aria-label="End of list markers"
					onClick={() => {
						setListItem(ndastvt_props.list_markers.length - 1);
					}}
				>
					<KeyboardDoubleArrowRight />
				</IconButton>
			</Box>
		</Box>
	);
};
