import { Box, BoxProps, Button, Chip, Grid, Typography } from '@mui/material';
import React, { FC, useEffect, useMemo } from 'react';
import { BNData, BinaryNode } from '../../data_processing/binary/BinaryNode';
import { DataMeta } from '../../data_processing/data_types';
import { LinkedDatePicker } from '../../input/LinkedDatePicker';
import { InfoDialog, InfoDialogProps } from '../../input/dialogs';
import { onlyRunOnceOnFirstRenderEffect } from '../../react/OnlyRunOnFirstRenderEffect';
import { RerenderForcer } from '../../react/RerenderForcer';
import { panel_styles } from '../db/controls';
import { Display_Main_Props } from '../display';
import {
	epochDayToLocalDate,
	localDateToEpochDay,
	shortDateString
} from '../staging/data_processing/processing01';
import { SolverCard } from './card';
import { PopoverButton, PopoverElement } from './popover';
import { FulfillmentDisplay } from './progress';

interface SolveCommand {
	daterangeupdate?: [number, number];
	start?: [number, number];
	stop?: boolean;
}

type RequestDetails = { f: number; u: number; i: number }[];

type ContradictionRankings = { name: string; count: number }[];

interface SolveMessage {
	unauthorized?: boolean;
	unauthenticated?: boolean;
	date_boundaries?: [number, number];
	current_date_range?: [number, number];
	solver_is_running?: boolean;
	restart_count?: number;
	contradictions?: number;
	solutions?: number;
	time?: number;
	opt_f?: [number, number];
	opt_r?: [number, number];
	assignable_count?: number;
	unlocked_assignable_count?: number;
	meta?: { worker_translator: number; column_meta: DataMeta };
	pertinent_requests?: { w: number; t: number; r: BNData; i: number[] }[];
	fulfilled_request_fraction?: number;
	update_count?: number;
	request_details?: RequestDetails;
	contradiction_rankings?: ContradictionRankings;
}

const control_style: React.CSSProperties = {
	marginTop: 10,
	marginBottom: 10,
	marginLeft: 5,
	marginRight: 5
};

const chipboxstyle: React.CSSProperties = {
	margin: '5px',
	height: '25px',
	flexGrow: 0,
	flexDirection: 'column',
	display: 'flex'
};
const chipstyle: React.CSSProperties = { flex: 1 };
const socket_closed_chip: React.ReactElement = (
	<Box sx={chipboxstyle}>
		<Chip sx={chipstyle} key="socket_status" label="Not Connected" color="error" />
	</Box>
);

const socket_open_chip: React.ReactElement = (
	<Box sx={chipboxstyle}>
		<Chip sx={chipstyle} key="socket_status" label="Connected" color="success" />
	</Box>
);

const ping_string: string = 'p';

class SolverSocket {
	private websocket: WebSocket;
	private last_state: SolveMessage = {} as SolveMessage;
	private rerender_forcer: RerenderForcer;
	private connection_status_chip: React.ReactElement;

	constructor(rerender_forcer: RerenderForcer) {
		this.rerender_forcer = rerender_forcer;
		this.connection_status_chip = socket_closed_chip;

		this.last_state = {
			date_boundaries: default_date_range(),
			current_date_range: default_date_range(),
			solver_is_running: false,
			restart_count: 0,
			contradictions: 0,
			time: 0,
			opt_f: [0, 1],
			opt_r: [0, 1],
			solutions: 0,
			assignable_count: 0,
			unlocked_assignable_count: 0,
			pertinent_requests: [],
			fulfilled_request_fraction: 0,
			update_count: 0
		};
	}

	public disconnect = () => {
		console.debug('Socket closing.');
		this.websocket.close();
	};

	public connect = () => {
		console.debug('Socket opening.');

		let wsprot: string;
		const protocol = window.location.protocol;
		if (protocol == 'http:') {
			wsprot = 'ws:';
		} else if (protocol == 'https:') {
			wsprot = 'wss:';
		}

		this.websocket = new WebSocket(wsprot + '//' + window.location.host + '/sock');
		this.websocket.addEventListener('open', (event) => {
			console.debug('Web socket opened', event);
			this.connection_status_chip = socket_open_chip;
			this.rerender_forcer.forceRerender();
		});
		this.websocket.addEventListener('message', (message) => {
			//console.debug("Message received:",message);
			if (message.data === ping_string) {
				//This is just a ping to keep the connection alive
				console.debug('Got a ping.');
				this.websocket.send(ping_string);
			} else {
				const update: SolveMessage = JSON.parse(message.data);
				console.debug(update);
				this.update(update);
			}
		});
		this.websocket.addEventListener('error', (message) => {
			console.error('Web socket error.', message);
		});
		this.websocket.addEventListener('close', (message) => {
			console.debug('Web socket closed.', message);
			this.connection_status_chip = socket_closed_chip;
			this.rerender_forcer.forceRerender();
			if (this.restart) {
				this.connect();
			}
		});

		this.websocket.readyState == WebSocket.OPEN;
	};

	private restart = true;
	public close = () => {
		this.restart = false;
		this.websocket.close();
	};

	private send = (message: string) => {
		if (!this.websocket) {
			console.error('Attempted to send on a nonexistent websocket.');
			this.connect();
		} else if (!this.websocket.OPEN) {
			console.error("Attempted to send on a websocket that isn't yet open.");
		} else {
			this.websocket.send(message);
		}
	};

	public send_daterangeupdate_command = () => {
		let message_object: SolveCommand = { daterangeupdate: this.last_state.current_date_range };
		let message: string = JSON.stringify(message_object);
		this.send(message);
	};

	public send_start_command: React.MouseEventHandler<HTMLButtonElement> = (
		event: React.MouseEvent<HTMLButtonElement, MouseEvent>
	) => {
		let message_object: SolveCommand = { start: this.last_state.current_date_range };
		let message: string = JSON.stringify(message_object);
		this.send(message);
	};

	public send_stop_command: React.MouseEventHandler<HTMLButtonElement> = (
		event: React.MouseEvent<HTMLButtonElement, MouseEvent>
	) => {
		let message_object: SolveCommand = {
			stop: true
		};
		let message: string = JSON.stringify(message_object);
		this.send(message);
	};

	private update = (update: SolveMessage) => {
		Object.assign(this.last_state, update);
		this.rerender_forcer.forceRerender();
	};

	public change_start_date = (date: Date) => {
		console.debug('Start date change', date);
		const current_range = this.last_state.current_date_range;
		current_range[0] = localDateToEpochDay(date);
		this.update({ current_date_range: current_range });
		this.send_daterangeupdate_command();
		console.debug('Date range update', current_range, this.last_state.current_date_range);
	};

	public change_end_date = (date: Date) => {
		console.debug('End date change', date);
		const current_range = this.last_state.current_date_range;
		current_range[1] = localDateToEpochDay(date);
		this.update({ current_date_range: current_range });
		this.send_daterangeupdate_command();
	};

	public getState() {
		return this.last_state;
	}

	public getStatusChip() {
		return this.connection_status_chip;
	}
}

const TopDisplayColumn: React.FC<any> = (props: BoxProps) => {
	const { sx, ...other } = props;
	return (
		<Box
			sx={{
				bgcolor: (theme) => (theme.palette.mode === 'dark' ? '#101010' : '#fff'),
				color: (theme) => (theme.palette.mode === 'dark' ? 'grey.300' : 'grey.800'),
				border: '1px solid',
				borderColor: (theme) => (theme.palette.mode === 'dark' ? 'grey.800' : 'grey.300'),
				p: 1,
				m: 1,
				borderRadius: 2,
				fontSize: '0.875rem',
				fontWeight: '700',
				height: 'auto',
				...sx,
				display: 'flex',
				flexDirection: 'column',
				minHeight: '0px'
			}}
			{...other}
		/>
	);
};

const buildRequestList = (state: SolveMessage, include_status: boolean) => {
	const items = [] as React.ReactElement[];
	for (let i = 0; i < state.pertinent_requests.length; i++) {
		const request = state.pertinent_requests[i];
		const worker_name = state.meta.column_meta.translators[state.meta.worker_translator][request.w];
		const bnode = new BinaryNode(undefined, state.meta.column_meta, request.r);
		console.debug('Building request list item.', state, state.pertinent_requests[i]);

		const grid_style: React.CSSProperties = {
			borderBottom: 'white',
			borderBottomStyle: 'dotted',
			borderBottomWidth: '1px',
			paddingBottom: '3px'
		};

		const grid_item_1 = (
			<Grid key="worker" item xs={1} display="flex" alignItems="center" justifyContent="left">
				<Typography>{worker_name}</Typography>
			</Grid>
		);

		const request_dates: React.ReactNode[] = [];
		for (const request_epoch_day of request.i) {
			request_dates.push(
				<Typography key={request_epoch_day}>
					{shortDateString(epochDayToLocalDate(request_epoch_day))}
				</Typography>
			);
		}
		const dates_pop = <Box>{request_dates}</Box>;

		let type_content: string;
		let date_text: string;
		if (request.t == 0) {
			type_content = 'Singular Request';
			date_text = request.i.length + ' Entailed Dates';
		} else {
			type_content = 'Recurring Request';
			date_text = request.i.length + ' Instance Dates';
		}

		const grid_item_2 = (
			<Grid key="dates" item xs={1} display="flex" alignItems="center" justifyContent="center">
				<PopoverButton button_text={date_text} pop={dates_pop} />
			</Grid>
		);

		const grid_item_3 = (
			<Grid
				key="request_details"
				item
				xs={1}
				display="flex"
				alignItems="center"
				justifyContent="center"
			>
				<PopoverButton
					button_text={type_content}
					pop={bnode.createDisplayNode(undefined, undefined).custom_display}
				/>
			</Grid>
		);

		if (include_status && state.request_details != undefined) {
			const status = state.request_details[i];
			const fulfilled_percentage = Math.round(status.f * 100) + '%';
			const unfulfilled_percentage = Math.round(status.u * 100) + '%';
			const indeterminant_percentage = Math.round(status.i * 100) + '%';
			items.push(
				<Grid style={grid_style} key={'r' + i} container item spacing={1} columns={{ xs: 4 }}>
					{grid_item_1}
					{grid_item_2}
					{grid_item_3}
					<Grid
						key="fulfillment"
						item
						xs={1}
						display="flex"
						alignItems="center"
						justifyContent="center"
					>
						<PopoverElement
							main={<FulfillmentDisplay min={status.f} max={status.f + status.i} />}
							pop={
								<Grid container direction="column">
									<Grid item>
										<Typography>{fulfilled_percentage} fulfilled</Typography>
									</Grid>
									<Grid item>
										<Typography>{unfulfilled_percentage} unfulfilled</Typography>
									</Grid>
									<Grid item>
										<Typography>{indeterminant_percentage} indeterminant</Typography>
									</Grid>
								</Grid>
							}
						/>
					</Grid>
				</Grid>
			);
		} else {
			items.push(
				<Grid style={grid_style} key={'r' + i} container item spacing={1} columns={{ xs: 3 }}>
					{grid_item_1}
					{grid_item_2}
					{grid_item_3}
				</Grid>
			);
		}
	}
	return (
		<Grid container direction="column" spacing={1} style={{ marginLeft: '0px', marginTop: '0px' }}>
			{items}
		</Grid>
	);
};

const default_date_range = () => {
	const d = new Date();
	return create_date_range(d, d);
};
const create_date_range: (start: Date, stop: Date) => [number, number] = (
	start: Date,
	stop: Date
) => {
	return [localDateToEpochDay(start), localDateToEpochDay(stop)];
};
export const SolveDisplay: FC<Display_Main_Props> = (display_props: Display_Main_Props) => {
	const rerender_forcer = new RerenderForcer();

	const solver_socket: SolverSocket = useMemo(() => {
		return new SolverSocket(rerender_forcer);
	}, []);

	useEffect(() => {
		solver_socket.connect();
		return () => {
			solver_socket.disconnect();
		};
	}, []);

	const state = solver_socket.getState();

	console.debug('Solve Display', display_props, state);

	const start_date = () => {
		return epochDayToLocalDate(state.current_date_range[0]);
	};
	const end_date = () => {
		return epochDayToLocalDate(state.current_date_range[1]);
	};
	const first_date = () => {
		return epochDayToLocalDate(state.date_boundaries[0]);
	};
	const last_date = () => {
		return epochDayToLocalDate(state.date_boundaries[1]);
	};

	onlyRunOnceOnFirstRenderEffect(
		() => {},
		() => {
			solver_socket.close();
		}
	);

	if (state.unauthenticated) {
		const props: InfoDialogProps = {
			app: display_props.app,
			title: 'Unauthenticated',
			message: 'User is not authorized. Websocket closed.',
			confirm_button_text: 'Okay'
		};
		display_props.app.showSuperDialog(InfoDialog(props));
		state.unauthenticated = undefined;
		solver_socket.close();
	} else if (state.unauthorized) {
		const props: InfoDialogProps = {
			app: display_props.app,
			title: 'Unauthorized Operation',
			message: 'User is not authorized for this operation.',
			confirm_button_text: 'Okay'
		};
		display_props.app.showSuperDialog(InfoDialog(props));
		state.unauthorized = undefined;
	}

	let model_display = <TopDisplayColumn />;
	if (
		state.assignable_count != undefined &&
		state.unlocked_assignable_count != undefined &&
		state.pertinent_requests != undefined
	) {
		model_display = (
			<TopDisplayColumn>
				<SolverCard
					header="Date Range"
					contents={[start_date().toDateString() + ' - ' + end_date().toDateString()]}
				/>
				<SolverCard
					header="Assignables"
					contents={[
						state.unlocked_assignable_count + ' of ' + state.assignable_count + ' unlocked.'
					]}
				/>
				<SolverCard
					header="Requests"
					contents={[state.pertinent_requests.length + ' pertinent requests.']}
					collapse_items={[buildRequestList(state, false)]}
				/>
			</TopDisplayColumn>
		);
	}

	let solver_display: React.ReactElement = <TopDisplayColumn />;
	if (
		state.solver_is_running &&
		state.restart_count != undefined &&
		state.contradictions != undefined &&
		state.solutions != undefined
	) {
		let contraindication_rankings_element = null;
		if (state.contradiction_rankings != undefined) {
			let ranking_elements = [] as string[];
			for (const cir of state.contradiction_rankings) {
				ranking_elements.push(cir.name + '(' + cir.count + ')');
			}

			contraindication_rankings_element = (
				<SolverCard header={'Contradiction Ranking'} collapse_items={ranking_elements} />
			);
		}

		const solver_progress_contents = [
			'Solution Count: ' + state.solutions,
			'Restarts: ' + state.restart_count,
			'Contradictions: ' + state.contradictions
		] as string[];
		if (state.time) {
			solver_progress_contents.push('Last solution found in ' + state.time + ' seconds.');
		}

		solver_display = (
			<TopDisplayColumn>
				<SolverCard header={'Solver Progress'} contents={solver_progress_contents} />
				{contraindication_rankings_element}
			</TopDisplayColumn>
		);
	} else {
		solver_display = (
			<TopDisplayColumn>
				<Typography>No solver running.</Typography>
			</TopDisplayColumn>
		);
	}

	let best_solution_display = <TopDisplayColumn />;
	if (
		state.time != undefined &&
		state.opt_f != undefined &&
		state.opt_r != undefined &&
		state.fulfilled_request_fraction != undefined &&
		state.pertinent_requests != undefined
	) {
		let request_details_element = null;
		if (state.request_details != undefined) {
			request_details_element = buildRequestList(state, true);
		}

		const fulfilled_request_percentage = Math.round(state.fulfilled_request_fraction * 100);

		best_solution_display = (
			<TopDisplayColumn>
				<SolverCard
					header="Optimization Objectives"
					contents={[
						<FulfillmentDisplay min={state.opt_f[0]} max={state.opt_f[1]} />,
						'Fairness component range: ' + state.opt_f[0] + '-' + state.opt_f[1],
						<FulfillmentDisplay min={state.opt_r[0]} max={state.opt_r[1]} />,
						'Request component range: ' + state.opt_r[0] + '-' + state.opt_r[1]
					]}
				/>
				<SolverCard
					header="Requests"
					contents={[fulfilled_request_percentage + '% of requests fulfilled.']}
					collapse_items={[request_details_element]}
				/>
			</TopDisplayColumn>
		);
	} else {
		best_solution_display = (
			<TopDisplayColumn>
				<Typography>No solution has been found yet.</Typography>
			</TopDisplayColumn>
		);
	}

	const main_display = (
		<Box
			key="main_display"
			height="100%"
			maxHeight="100%"
			sx={{ display: 'flex', flexDirection: 'column', flexGrow: 1 }}
		>
			{solver_socket.getStatusChip()}
			<Box
				key="panel display"
				width="100%"
				sx={{
					flexGrow: 1,
					overflowY: 'scroll',
					display: 'grid',
					gridTemplateColumns: 'repeat(3,1fr)',
					gridTemplateRows: '1fr'
				}}
			>
				{model_display}
				{solver_display}
				{best_solution_display}
			</Box>
		</Box>
	);

	const control_panel = (
		<Box
			key="control_panel"
			style={{ flexShrink: 1, paddingTop: 10, ...panel_styles }}
			display="flex"
			flexDirection="row"
			height="100%"
		>
			<Box display="flex" flexDirection="column" width="100%" overflow="scroll">
				<Box style={control_style}>
					<LinkedDatePicker
						id="start_date"
						label="Start Date"
						disabled={state.solver_is_running}
						date={start_date()}
						min_date={first_date()}
						max_date={last_date()}
						handler={(date: Date) => {
							solver_socket.change_start_date(date);
						}}
					/>
				</Box>
				<Box style={control_style}>
					<LinkedDatePicker
						id="end_date"
						label="End Date"
						disabled={state.solver_is_running}
						date={end_date()}
						min_date={first_date()}
						max_date={last_date()}
						handler={(date: Date) => {
							solver_socket.change_end_date(date);
						}}
					/>
				</Box>
				<Box
					display="flex"
					flexDirection="row"
					justifyContent="space-between"
					width="100%"
					style={control_style}
				>
					<Button
						variant="contained"
						disabled={state.solver_is_running}
						onClick={solver_socket.send_start_command}
					>
						Start
					</Button>
					<Button
						variant="contained"
						disabled={!state.solver_is_running}
						onClick={solver_socket.send_stop_command}
					>
						Stop
					</Button>
				</Box>
			</Box>
		</Box>
	);

	return (
		<Box display="flex" flexDirection="row" width="100%" height="100%">
			{main_display}
			{control_panel}
		</Box>
	);
};
