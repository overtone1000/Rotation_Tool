import {
	Box,
	Button,
	Dialog,
	DialogActions,
	DialogContent,
	DialogTitle,
	FormControl,
	Input,
	InputLabel
} from '@mui/material';
import React from 'react';
import { FC } from 'react';
import { getCSRFFormInput } from './csrf';

import { WrappedHook } from './react/WrappedHook';
//import { autoscheda_theme } from "./theming/theme";

import { findGetParameter } from './commons/DOMfunctions';
import { dialog_box_style } from './commons/styles';

interface RecoveryProps {}

export const passwordField = (
	label: string,
	name: string,
	hook: WrappedHook<string>,
	box_style: React.CSSProperties
) => {
	return (
		<FormControl style={box_style}>
			<InputLabel>{label}</InputLabel>
			<Input
				name={name}
				type="password"
				disabled={false}
				value={hook.get()}
				onChange={(e) => {
					hook.set(e.target.value);
				}}
			/>
		</FormControl>
	);
};

export class PasswordChanger {
	public new_password: WrappedHook<string> = new WrappedHook<string>('');
	public check_password: WrappedHook<string> = new WrappedHook<string>('');
	public change_disabled: WrappedHook<boolean> = new WrappedHook<boolean>(true);

	constructor() {
		this.new_password.side_effects.addSideEffect('pwcheck_new', this.checknew);
		this.check_password.side_effects.addSideEffect('pwcheck_check', this.checkcheck);
	}

	private pwcheck = (newval: string, otherhook: WrappedHook<string>) => {
		if (newval == '') {
			this.change_disabled.set(true);
			console.debug('Password is blank.');
			return;
		}
		let other = otherhook.get();
		if (newval !== other) {
			this.change_disabled.set(true);
			console.debug("Passwords don't match: " + newval + ',' + other);
			return;
		}
		this.change_disabled.set(false);
	};

	private checknew = (newval: string) => {
		return this.pwcheck(newval, this.check_password);
	};
	private checkcheck = (newval: string) => {
		return this.pwcheck(newval, this.new_password);
	};
}

export const AutoschedaAccountRecovery: FC<RecoveryProps> = () => {
	const changer = new PasswordChanger();

	const password_input_name = 'password';

	const account_recovery_submission = '/account_recovery_submission';
	const usernamefield = 'user';
	const tokenfield = 'token';

	let usernamevalue = findGetParameter(usernamefield);
	if (usernamevalue === undefined) {
		usernamevalue = '';
	}
	let tokenvalue = findGetParameter(tokenfield);
	if (tokenvalue === undefined) {
		tokenvalue = '';
	}

	console.debug('Window location', window.location);
	console.debug('User name: ' + usernamevalue);
	console.debug('Token: ' + tokenvalue);

	return (
		//<ThemeProvider theme={autoscheda_theme}>
		<Dialog
			open={true}
			fullWidth={false}
			maxWidth={false}
			disableEscapeKeyDown={true}
			fullScreen={false}
			aria-labelledby="form-dialog-title"
		>
			<form method="post" action={account_recovery_submission}>
				<DialogTitle key={'title'}>
					<Box display="flex" flexGrow={1} alignItems="center" justifyContent="center">
						Account Recovery
					</Box>
				</DialogTitle>
				<DialogContent key={'content'}>
					<Box
						display="flex"
						flexDirection="column"
						flexGrow={1}
						alignItems="center"
						justifyContent="center"
					>
						{passwordField(
							'New Password',
							password_input_name,
							changer.new_password,
							dialog_box_style
						)}
						{passwordField(
							'Re-enter Password',
							'check_pw_field',
							changer.check_password,
							dialog_box_style
						)}
					</Box>
					{getCSRFFormInput()}
					<Input type="hidden" name={usernamefield} value={usernamevalue} />
					<Input type="hidden" name={tokenfield} value={tokenvalue} />
				</DialogContent>
				<DialogActions key={'actions'}>
					<Box
						display="flex"
						width="100%"
						flexGrow={1}
						flexShrink={0}
						alignItems="center"
						justifyContent="center"
					>
						<Button
							disabled={changer.change_disabled.get()}
							type="submit"
							key={'save_button'}
							color="primary"
							variant="contained"
						>
							Change Password
						</Button>
					</Box>
				</DialogActions>
			</form>
		</Dialog>
		//</ThemeProvider>
	);
};
