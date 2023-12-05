import React, { useState } from 'react';
import { TextField, TextFieldProps } from '@mui/material';
import { DatePicker, LocalizationProvider } from '@mui/x-date-pickers';
import { AdapterDateFns } from '@mui/x-date-pickers/AdapterDateFns';

export const TestDatePicker = () => {
	const [date, changeDate] = useState<Date>(new Date());

	const onChange = (newdate: Date, keyboardInputValue?: string) => {
		changeDate(newdate);
	};

	const renderInput = (params: TextFieldProps) => {
		return <TextField {...params} />;
	};

	const retval = (
		<LocalizationProvider dateAdapter={AdapterDateFns}>
			<DatePicker
				label={'Test'}
				value={date}
				onChange={onChange}
				renderInput={renderInput}
				inputFormat="MM/dd/yyyy"
			/>
		</LocalizationProvider>
	);
	console.debug(retval);
	return retval;
};
