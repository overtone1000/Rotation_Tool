import CloseIcon from '@mui/icons-material/Close';
import {
	Box,
	Button,
	ButtonBase,
	Dialog,
	DialogActions,
	DialogContent,
	DialogTitle,
	FormControl,
	IconButton,
	Input,
	InputLabel,
	Paper,
	ThemeProvider,
	Typography
} from '@mui/material';
import React, { FC } from 'react';
import { dialog_box_style } from './commons/styles';
import { csrf_token_header, getCSRFFormInput, getCSRFToken } from './csrf';
import { WrappedHook } from './react/WrappedHook';
import { autoscheda_theme } from './theming/theme';

const nomal_google_icon = new URL(
	'../resources/images/google/btn_google_signin_dark_normal_web.png',
	import.meta.url
);

interface LoginProps {}

enum Context {
	login,
	recovery
}

interface AuthenticationRequest {
	user: string;
}

interface AuthenticationResponse {
	message: string;
}

export const message_element = (message: string) => {
	if (message === null || message === undefined) {
		return null;
	} else {
		return (
			<Box
				display="flex"
				flexGrow={1}
				flexShrink={1}
				alignItems="center"
				justifyContent="center"
				style={{ padding: 16 }}
			>
				<Paper variant="elevation" elevation={2} style={{ width: '300px' }}>
					<Typography>{message}</Typography>
				</Paper>
			</Box>
		);
	}
};

export const AutoschedaLogin: FC<LoginProps> = () => {
	console.log('Rendering Login Dialog');

	let username: WrappedHook<string> = new WrappedHook<string>('');
	let password: WrappedHook<string> = new WrappedHook<string>('');

	let context: WrappedHook<Context> = new WrappedHook<Context>(Context.login);
	context.side_effects.addSideEffect('clearing message', (new_context) => {
		message.set(null);
	}); //clear the message when the context changes

	let message: WrappedHook<string> = new WrappedHook<string>(null);

	const loginurl = '/credential_submission';
	const user_input_name = 'user';
	const password_input_name = 'password';

	const recoveryurl = '/account_recovery_request';

	const google_oath2 = '/oauth2/authorization/google';

	const sendPOST = function (location: string, request: AuthenticationRequest) {
		const headers = {};
		headers['Content-Type'] = 'application/json';
		headers[csrf_token_header] = getCSRFToken();

		const req_init: RequestInit = {
			method: 'POST',
			credentials: 'include',
			body: JSON.stringify(request)
		};
		req_init.headers = headers;

		console.debug('Sending request.', req_init);
		window
			.fetch(location, req_init)
			.then((response) => {
				console.debug('Response received.', response);
				response
					.json()
					.then((parsed: AuthenticationResponse) => {
						console.debug('Response parsed.', parsed);
						message.set(parsed.message);
					})
					.catch((error) => {
						console.error('Parse error.', error);
					});
			})
			.catch((error) => {
				console.error('Fetch error.', error);
			});
	};

	const requestAccountRecovery = () => {
		const data = { user: username.get() };
		console.debug('Account recovery request for', data);
		sendPOST(recoveryurl, data);
	};

	const username_input = (
		<FormControl style={dialog_box_style}>
			<InputLabel>E-mail</InputLabel>
			<Input
				name={user_input_name}
				type="email"
				disabled={false}
				value={username.get()}
				onChange={(e) => {
					console.debug('Username is being set to', e.target.value);
					username.set(e.target.value);
				}}
			/>
		</FormControl>
	);

	const onClose = (event: {}, reason: 'backdropClick' | 'escapeKeyDown') => {
		if (reason !== 'backdropClick') {
			//handleClose(event, reason);
		}
	};

	const onBackdropClick = () => {};

	const login: React.ReactElement = (
		<ThemeProvider theme={autoscheda_theme}>
			<Dialog
				open={true}
				fullWidth={false}
				maxWidth={false}
				onBackdropClick={onBackdropClick}
				disableEscapeKeyDown={true}
				fullScreen={false}
				aria-labelledby="form-dialog-title"
			>
				<form method="post" action={loginurl}>
					<DialogTitle key={'title'}>
						<Box display="flex" flexGrow={1} alignItems="center" justifyContent="center">
							Autoscheda Login
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
							{username_input}
							<FormControl style={dialog_box_style}>
								<InputLabel>Password</InputLabel>
								<Input
									name={password_input_name}
									type="password"
									disabled={false}
									value={password.get()}
									onChange={(e) => {
										password.set(e.target.value);
									}}
								/>
							</FormControl>
						</Box>
						{getCSRFFormInput()}
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
								name="submit"
								type="submit"
								key={'save_button'}
								color="primary"
								variant="contained"
							>
								Login
							</Button>
						</Box>
					</DialogActions>
				</form>
				<Box
					display="flex"
					width="100%"
					flexGrow={1}
					flexShrink={0}
					alignItems="center"
					justifyContent="center"
				>
					<Button
						color="secondary"
						onClick={() => {
							context.set(Context.recovery);
						}}
					>
						Account Recovery
					</Button>
				</Box>
				<Box
					display="flex"
					width="100%"
					flexGrow={1}
					flexShrink={0}
					alignItems="center"
					justifyContent="center"
				>
					<Button
						color="primary"
						onClick={() => {
							window.location.href = '/docs';
						}}
					>
						Read the Docs
					</Button>
				</Box>
				{message_element(message.get())}
				<Box
					display="flex"
					width="100%"
					flexGrow={1}
					flexShrink={0}
					alignItems="center"
					justifyContent="center"
					style={{ marginTop: 16, borderTop: '1px solid', paddingTop: 16, paddingBottom: 16 }}
				>
					<ButtonBase key="google_oauth2" href={google_oath2}>
						<img src={nomal_google_icon.toString()}></img>
					</ButtonBase>
				</Box>
			</Dialog>
		</ThemeProvider>
	);

	const recovery: React.ReactElement = (
		<ThemeProvider theme={autoscheda_theme}>
			<Dialog
				open={true}
				fullWidth={false}
				maxWidth={false}
				onBackdropClick={onBackdropClick}
				disableEscapeKeyDown={true}
				fullScreen={false}
				aria-labelledby="form-dialog-title"
			>
				<form method="post" action={recoveryurl}>
					<DialogTitle key={'title'}>
						<Box display="flex" flexGrow={1} alignItems="center" justifyContent="center">
							<Box display="flex" flexGrow={1} alignItems="center" justifyContent="center">
								Autoscheda Account Recovery
							</Box>
							<Box display="flex" flexShrink={1} alignItems="left">
								<IconButton
									onClick={() => {
										context.set(Context.login);
									}}
								>
									<CloseIcon></CloseIcon>
								</IconButton>
							</Box>
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
							{username_input}
						</Box>
						{getCSRFFormInput()}
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
								key={'save_button'}
								color="primary"
								variant="contained"
								onClick={requestAccountRecovery}
							>
								Recover Password
							</Button>
						</Box>
					</DialogActions>
				</form>
				{message_element(message.get())}
			</Dialog>
		</ThemeProvider>
	);

	switch (context.get()) {
		case Context.login:
			return login;
		case Context.recovery:
			return recovery;
	}
};
