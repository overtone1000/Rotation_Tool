import { Box, TextField, TextFieldProps, Typography } from '@mui/material';
import { DatePicker, LocalizationProvider } from '@mui/x-date-pickers';
import { AdapterDateFns } from '@mui/x-date-pickers/AdapterDateFns';
import { DateValidationError } from '@mui/x-date-pickers/internals/hooks/validation/useDateValidation';
import { isValid } from 'date-fns';
import React from 'react';
import { RerenderForcer } from '../react/RerenderForcer';
import { ObjectKeyPair } from './form_elements';

interface CommonProps {
	id: string;
	label: string;
	disabled: boolean;
	min_date?: Date;
	max_date?: Date;
}

export interface LDP_Props extends CommonProps {
	date: Date;
	handler: (date: Date) => void;
}

export interface LDP_Props_OKP extends CommonProps {
	date_object_key_pair: ObjectKeyPair<Date>;
	side_effect?: (date: Date) => void;
}

export const dateOnly = (date: Date) => {
	return new Date(date.getFullYear(), date.getMonth(), date.getDate());
};

export const processDateChange = (newdate: Date, keyboardInputValue?: string) => {
	//console.debug("Changedate",newdate,keyboardInputValue);
	if (newdate === null) {
		return null;
	} else if (isValid(newdate)) {
		//Make sure the full year has been typed in and only call the handler when all four digits of the year have been typed. Will only work until the year 10000 AD. So, put it on your calendar to come back and improve this code at that time.
		if (keyboardInputValue === undefined || keyboardInputValue.length >= 10) {
			//Less ugly way
			return dateOnly(newdate);
		}
	}
	return undefined;
};

export const LinkedDatePicker = (props: LDP_Props) => {
	//console.debug("LinkedDatePicker",props);
	if (!props) {
		console.debug('Null or undefined props!');
		console.trace();
		return <Typography>Huh?</Typography>;
	}

	const changeDate = (newdate: Date, keyboardInputValue?: string) => {
		const processed_date = processDateChange(newdate, keyboardInputValue);
		if (processed_date) {
			props.handler(processed_date);
		}
	};

	const renderInput = (params: TextFieldProps) => {
		return <TextField id={props.label + ' tb'} {...params} />;
	};

	return render(
		props.id,
		props.label,
		props.date,
		props.min_date,
		props.max_date,
		changeDate,
		renderInput,
		props.disabled
	);
};

export const renderDatePickerInput = (id: string) => {
	return (params: TextFieldProps) => {
		return <TextField id={id} {...params} />;
	};
};

export const LinkedDatePicker_OKP = (props: LDP_Props_OKP) => {
	//console.debug("LinkedDatePicker",props);
	if (!props) {
		console.debug('Null or undefined props!');
		console.trace();
		return <Typography>Huh?</Typography>;
	}

	const forcer = new RerenderForcer();

	const changeDate = (newdate: Date, keyboardInputValue?: string) => {
		const processed_date = processDateChange(newdate, keyboardInputValue);
		if (processed_date) {
			props.date_object_key_pair.set(processed_date);
			if (props.side_effect) {
				props.side_effect(processed_date);
			}
			forcer.forceRerender();
		}
	};

	return render(
		props.id,
		props.label,
		props.date_object_key_pair.get(),
		props.min_date,
		props.max_date,
		changeDate,
		renderDatePickerInput(props.id),
		props.disabled
	);
};

const render = (
	id: string,
	label: string,
	date: Date,
	minDate: Date,
	maxDate: Date,
	onChange: (newdate: Date, keyboardInputValue?: string) => void,
	renderInput: (
		params: TextFieldProps
	) => React.ReactElement<any, string | React.JSXElementConstructor<any>>,
	disabled: boolean
) => {
	const onAccept = (date: Date) => {
		console.debug('New date accepted', date);
	};

	//console.debug("Rendering DatePicker with date",date,minDate,maxDate);
	const retval = (
		<LocalizationProvider dateAdapter={AdapterDateFns}>
			<Box>
				<DatePicker
					label={label}
					value={date}
					onChange={onChange}
					renderInput={renderInput}
					minDate={minDate}
					maxDate={maxDate}
					inputFormat="MM/dd/yyyy"
					onAccept={onAccept}
					disabled={disabled}
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
			</Box>
		</LocalizationProvider>
	);
	//console.debug(retval);
	return retval;
};
