import * as React from 'react';
import Stack from '@mui/material/Stack';
import Button from '@mui/material/Button';
import Snackbar from '@mui/material/Snackbar';
import MuiAlert, { AlertProps } from '@mui/material/Alert';
import { Box, SnackbarContent, Typography } from '@mui/material';
import { WrappedHook } from '../react/WrappedHook';

function randomNotification() {
	const notifTitle = 'Test notification';
	const notifBody = 'Notification Worked!';
	const notifImg =
		'https://res.cloudinary.com/softwarepundit/image/upload/c_limit,dpr_1.0,f_auto,h_900,q_auto,w_900/v1/software/calendar-icon';
	const options = {
		body: notifBody,
		icon: notifImg
	};
	new Notification(notifTitle, options);
	setTimeout(randomNotification, 10000);
}

const browserSupportsNotifications = () => {
	const retval = Notification !== undefined && Notification !== null;
	if (retval) {
		console.debug('Browser supports notifications.');
	}
	return retval;
};
const notificationsAreAllowed = () => {
	console.debug('Current browser notifications permission:', Notification.permission);
	return Notification.permission === 'granted';
};

export const snackbar = () => {
	const shown = new WrappedHook<boolean>(
		browserSupportsNotifications() && !notificationsAreAllowed()
	); //Don't need to prompt if notifications are already allowed.

	const handleClose = (event?: React.SyntheticEvent | Event, reason?: string) => {
		if (reason === 'clickaway') {
			return;
		}

		shown.set(false);
	};

	const notifications_requested_by_user = () => {
		shown.set(false);
		Notification.requestPermission().then((value: NotificationPermission) => {
			console.debug('Permissions are now ', value);
		});
	};

	const action = (
		<Box>
			<Button
				color="primary"
				variant="contained"
				size="small"
				onClick={notifications_requested_by_user}
			>
				Yes
			</Button>
			<Button
				color="secondary"
				variant="contained"
				size="small"
				onClick={() => {
					shown.set(false);
				}}
			>
				No
			</Button>
		</Box>
	);

	return (
		<Snackbar
			open={shown.get()}
			autoHideDuration={30000}
			onClose={handleClose}
			anchorOrigin={{ vertical: 'bottom', horizontal: 'center' }}
		>
			<SnackbarContent message="Allow notifications?" action={action} />
		</Snackbar>
	);
};
