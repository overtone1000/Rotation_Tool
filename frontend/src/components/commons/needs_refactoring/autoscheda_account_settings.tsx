import ArrowBackIcon from '@mui/icons-material/ArrowBack';
import LockIcon from '@mui/icons-material/Lock';
import {
	Box,
	Button,
	CssBaseline,
	Divider,
	Drawer,
	List,
	ListItem,
	ListItemIcon,
	ListItemText,
	ThemeProvider
} from '@mui/material';
import { makeStyles } from '@mui/styles';
import React, { FC } from 'react';
import {
	ASRequest,
	Command_Actions,
	Command_Contexts,
	ResponseHandler,
	sendPOST
} from './ajax/commands_generic';
import { default_autoscheda_styles } from './autoscheda_core';
import { message_element } from './autoscheda_login';
import { PasswordChanger, passwordField } from './autoscheda_recovery_submission';
import { dialog_box_style } from './commons/styles';
import { WrappedHook } from './react/WrappedHook';
import { autoscheda_theme } from './theming/theme';

interface SettingsProps {}

enum Context {
	password
}

const drawerWidth = 240;

const makeStyle = makeStyles((theme) => ({
	root: {
		display: 'flex'
	},
	appBar: {
		width: `calc(100% - ${drawerWidth}px)`,
		marginLeft: drawerWidth
	},
	drawer: {
		width: drawerWidth,
		flexShrink: 0
	},
	drawerPaper: {
		width: drawerWidth
	}
	// necessary for content to be below app bar
	/*
  toolbar: theme.mixins.toolbar,
  content: {
    flexGrow: 1,
    backgroundColor: theme.palette.background.default,
    padding: theme.spacing(3),
  },
  */
}));

interface SettingsChangeResponse {
	success: boolean;
}

const Password: FC<SettingsProps> = () => {
	const changer = new PasswordChanger();
	const message = new WrappedHook<string>('');
	const password_input_name = 'password';
	const submit = () => {
		//sendPOST = function(
		//request:ASRequest,
		//handler:ResponseHandler=defaultResponseHandler,
		//nowAllowedHandler:()=>any)
		const request: ASRequest = {
			action: Command_Actions.modify_prefix,
			context: Command_Contexts.user_password,
			parameters: {
				password: changer.new_password.get()
			}
		};

		const handler: ResponseHandler = (response: SettingsChangeResponse) => {
			if (response.success === null || response.success === undefined) {
				message.set('Malformed response from the server.');
			} else if (response.success) {
				message.set('Password update success.');
			} else {
				message.set('Password update failed.');
			}
		};

		const notAllowedHandler = () => {
			message.set('User not authorized.');
		};

		sendPOST(request, handler, notAllowedHandler, () => {});
	};

	return (
		<Box>
			<Box
				display="flex"
				flexDirection="column"
				flexGrow={1}
				alignItems="center"
				justifyContent="center"
			>
				{passwordField('New Password', password_input_name, changer.new_password, dialog_box_style)}
				{passwordField(
					'Re-enter Password',
					'check_pw_field',
					changer.check_password,
					dialog_box_style
				)}
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
					disabled={changer.change_disabled.get()}
					key={'save_button'}
					color="primary"
					variant="contained"
					onClick={submit}
				>
					Change Password
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
				{message_element(message.get())}
			</Box>
		</Box>
	);
};

export const AutoschedaAccountSettings: FC<SettingsProps> = () => {
	let context: WrappedHook<Context> = new WrappedHook<Context>(Context.password);

	const classes = makeStyle();

	const listitemselectedvals = {
		back: false,
		password: false
	};

	let subdisplay: React.ReactElement;
	switch (context.get()) {
		case Context.password:
			subdisplay = <Password />;
			listitemselectedvals.password = true;
			break;
	}

	// can use <Divider /> in the Drawer
	const display: React.ReactElement = (
		<ThemeProvider theme={autoscheda_theme}>
			<Box
				display="flex"
				flexDirection="row"
				width="100vw"
				height="100vh"
				overflow="hidden"
				style={default_autoscheda_styles}
			>
				<CssBaseline />
				<Drawer
					className={classes.drawer}
					variant="permanent"
					classes={{
						paper: classes.drawerPaper
					}}
					anchor="left"
				>
					<List>
						<ListItem
							button
							key="Back"
							onClick={() => {
								window.location.assign('/');
							}}
						>
							<ListItemIcon key="icon">
								<ArrowBackIcon />
							</ListItemIcon>
							<ListItemText key="text" primary="Exit" />
						</ListItem>
						<Divider />
						<ListItem button key="Password" selected={listitemselectedvals.password}>
							<ListItemIcon key="icon">
								<LockIcon />
							</ListItemIcon>
							<ListItemText key="text" primary="Password" />
						</ListItem>
					</List>
				</Drawer>
				<Box flexGrow={1} flexShrink={1} display="flex" flexDirection="row">
					{subdisplay}
				</Box>
			</Box>
		</ThemeProvider>
	);

	return display;
};
