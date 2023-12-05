import { formatISO, parseISO } from 'date-fns';
import { formStyle_cb, formStyle_generic_inline } from '../commons/styles';
import {
	epochDayToLocalDate,
	localDateToEpochDay
} from '../displays/staging/data_processing/processing01';
import { LDP_Props_OKP, LinkedDatePicker_OKP } from './LinkedDatePicker';

export class ObjectKeyPair<T> {
	object: { [i: string]: T };
	key: string;
	constructor(object: { [i: string]: T }, key: string) {
		this.object = object;
		this.key = key;
	}
	get(): T {
		return this.object[this.key];
	}
	set(value: T) {
		console.debug('Setting generic to ' + value);
		this.object[this.key] = value;
	}
}

export class ObjectKeyPair_Int extends ObjectKeyPair<number> {
	set(value: any) {
		console.debug('Setting integer to ' + value);
		this.object[this.key] = parseInt(value);
	}
}

export class ObjectKeyPair_Float extends ObjectKeyPair<number> {
	set(value: any) {
		console.debug('Setting float to ' + value);
		this.object[this.key] = parseFloat(value);
	}
}

export interface ValueBackedFormControl_Props<T> {
	id: string;
	label: string;
	data_type: string;
	disabled: boolean;
	obkey: ObjectKeyPair<T>;
	initial_value: T; //Necessary to impose external changes on the form
	changeHandler: undefined | (() => void);
}

export interface ValueBackedFormControl_State {
	rerenderer: boolean;
}

export class ValueBackedFormControl<
	V,
	T extends ValueBackedFormControl_Props<V>,
	U extends ValueBackedFormControl_State
> extends React.Component<T, U> {
	constructor(props: T) {
		super(props);
		console.debug('Constructing VBFC', props);
		this.state = {
			rerenderer: false
		} as U;
	}

	handleValChange = (evt: any) => {
		const value = evt.target.value;
		console.debug('handleValChange', value);
		this.setVal(value);
	};

	setVal(new_val: V) {
		this.props.obkey.set(new_val); //this saves the value back to its source
		this.setState({ rerenderer: !this.state.rerenderer }); //this is needed to rerender
		if (this.props.changeHandler !== undefined) {
			this.props.changeHandler();
		}
	}

	render() {
		return (
			<FormControl style={formStyle_generic_inline}>
				<InputLabel>{this.props.label}</InputLabel>
				<Input
					id={this.props.id}
					type={this.props.data_type}
					disabled={this.props.disabled}
					//value={this.state.value}
					value={this.props.obkey.get()}
					onChange={this.handleValChange}
				/>
			</FormControl>
		);
	}
}

class BasicControl<T> extends ValueBackedFormControl<
	T,
	ValueBackedFormControl_Props<T>,
	ValueBackedFormControl_State
> {}

export const createSimpleBooleanControl = (
	id: string,
	label: string,
	key: string,
	disabled: boolean,
	value: boolean,
	handler: (event: React.ChangeEvent<HTMLInputElement>, checked: boolean) => void
) => {
	const cb = (
		<Checkbox id={id} color="primary" disabled={disabled} checked={value} onChange={handler} />
	);

	return (
		<FormControl key={key} style={formStyle_cb}>
			<FormControlLabel
				id={id + '_label'}
				style={{ width: '100%', height: '100%' }}
				control={cb}
				label={label}
				labelPlacement="start"
			/>
		</FormControl>
	);
};

class BooleanControl extends ValueBackedFormControl<
	boolean,
	ValueBackedFormControl_Props<boolean>,
	ValueBackedFormControl_State
> {
	/*
    <InputLabel>{this.props.label}</InputLabel>
    <Input 
        type={this.props.data_type} 
        disabled={this.props.disabled} 
        value={this.state.value} 
        onChange = {this.handleValChange}
    />
    */

	handleValChange = (evt: any) => {
		const value = evt.target.checked;
		console.debug('handleValChange', value);
		this.setVal(value);
	};

	render() {
		return createSimpleBooleanControl(
			this.props.id,
			this.props.label,
			'boolctl',
			this.props.disabled,
			this.props.obkey.get(),
			this.handleValChange
		);
	}
}

interface ControlProps {
	id: string;
	label: string;
	disabled: boolean;
	object: { [i: string]: number };
	object_key: string;
	changeHandler: undefined | (() => void);
}

export const createGenericControl = function <T>(
	id: string,
	label: string,
	data_type: string,
	disabled: boolean,
	object: { [i: string]: T },
	key: string,
	changeHandler: undefined | (() => void)
): React.ReactNode {
	const obkey = new ObjectKeyPair<T>(object, key);
	return createControl(id, label, data_type, disabled, obkey, changeHandler);
};
export const createIntegerControl = function (
	id: string,
	label: string,
	disabled: boolean,
	object: { [i: string]: number },
	key: string,
	changeHandler: undefined | (() => void)
): React.ReactNode {
	const obkey = new ObjectKeyPair_Int(object, key);
	return createControl(id, label, 'number', disabled, obkey, changeHandler);
};
export const createFloatControl = function (
	id: string,
	label: string,
	disabled: boolean,
	object: { [i: string]: number },
	key: string,
	changeHandler: undefined | (() => void)
): React.ReactNode {
	const obkey = new ObjectKeyPair_Float(object, key);
	return createControl(id, label, 'number', disabled, obkey, changeHandler);
};
export const DisableableDoubleControl: React.FC<ControlProps> = (props: ControlProps) => {
	const obkey = new ObjectKeyPair_Float(props.object, props.object_key);
	console.debug('Creating DisableableDoubleControl', props, obkey, obkey.get());
	const disabled = new WrappedHook<boolean>(obkey.get() == null);

	const disabled_input_change_handler = (
		event: React.ChangeEvent<HTMLInputElement>,
		checked: boolean
	) => {
		disabled.set(!checked);
		obkey.set(null);
		if (props.changeHandler !== undefined) {
			props.changeHandler();
		}
	};

	return (
		<FormControl style={{ flexDirection: 'row' }}>
			{createSimpleBooleanControl(
				props.id + '_b',
				props.label + ' Enabled',
				props.id + '_b',
				false,
				!disabled.get(),
				disabled_input_change_handler
			)}
			{createFloatControl(
				props.id + '_v',
				props.label + ' Value',
				disabled.get(),
				props.object,
				props.object_key,
				props.changeHandler
			)}
		</FormControl>
	);
};

export const createEpochDayControl = function (
	id: string,
	label: string,
	disabled: boolean,
	object: { [i: string]: number },
	key: string,
	changeHandler: undefined | (() => void)
): React.ReactNode {
	const obkey = new ObjectKeyPair<number>(object, key);

	const date: Date = epochDayToLocalDate(obkey.get());
	const sub_key: string = 'd';
	const sub_object = {};
	sub_object[sub_key] = date;

	const sub_handler = () => {
		const epoch_day = localDateToEpochDay(sub_object[sub_key]);
		obkey.set(epoch_day);
		console.debug('Epoch day set.', obkey);
		if (changeHandler) {
			changeHandler();
		}
	};

	return createDateControl(id, label, disabled, sub_object, sub_key, sub_handler);
};

type DateObject = { [i: string]: Date };
export const createDateStringControl = function (
	id: string,
	label: string,
	disabled: boolean,
	object: { [i: string]: string },
	key: string,
	changeHandler: undefined | (() => void)
): React.ReactNode {
	const obkey = new ObjectKeyPair<string>(object, key);

	const date: Date = parseISO(obkey.get());
	const sub_key: string = 'd';
	const sub_object = {} as DateObject;
	sub_object[sub_key] = date;

	const sub_handler = () => {
		const currentdate = sub_object[sub_key];
		const datestring: string = formatISO(currentdate, { representation: 'date' });
		obkey.set(datestring);
		console.debug('Date string set.', obkey);
		if (changeHandler) {
			changeHandler();
		}
	};

	return createDateControl(id, label, disabled, sub_object, sub_key, sub_handler);
};

export const createDateControl = function (
	id: string,
	label: string,
	disabled: boolean,
	object: { [i: string]: Date },
	key: string,
	changeHandler: undefined | (() => void)
): React.ReactNode {
	const obkey = new ObjectKeyPair<Date>(object, key);

	console.debug('Passed in date=', obkey.get());

	const handler = (date: Date) => {
		obkey.set(date);
		console.debug('Date control set.', obkey);
		if (changeHandler) {
			changeHandler();
		}
	};

	const props: LDP_Props_OKP = {
		id: id,
		label: label,
		date_object_key_pair: obkey,
		side_effect: handler,
		disabled: disabled
	};

	console.debug('Creating date picker', props);
	const ldp = React.createElement(LinkedDatePicker_OKP, props);

	return <Box style={formStyle_generic_inline}>{ldp}</Box>;
};

export const createBooleanControl = function (
	id: string,
	label: string,
	disabled: boolean,
	object: { [i: string]: boolean },
	key: string,
	changeHandler: undefined | (() => void)
): React.ReactNode {
	const obkey = new ObjectKeyPair<boolean>(object, key);
	return (
		<BooleanControl
			id={id}
			key={obkey.key}
			label={label}
			data_type={undefined}
			disabled={disabled}
			obkey={obkey}
			initial_value={obkey.get()}
			changeHandler={changeHandler}
		/>
	);
};

const createControl = function <T>(
	id: string,
	label: string,
	data_type: string,
	disabled: boolean,
	obkey: ObjectKeyPair<T>,
	changeHandler: undefined | (() => void)
): React.ReactNode {
	console.debug('Creating basic control', obkey);
	return (
		<BasicControl<T>
			id={id}
			key={obkey.key}
			label={label}
			data_type={data_type}
			disabled={disabled}
			obkey={obkey}
			initial_value={obkey.get()}
			changeHandler={changeHandler}
		/>
	);
};
