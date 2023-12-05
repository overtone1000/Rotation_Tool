import { Box, Chip, Popover } from '@mui/material';
import React, { FC } from 'react';
import { WrappedHook } from '../../react/WrappedHook';

interface PopoverProps {
	main: React.ReactNode;
	pop: React.ReactNode;
}

export const PopoverElement: FC<PopoverProps> = (props: PopoverProps) => {
	const show_popover = new WrappedHook<HTMLElement | null>(null);
	const handlePopoverOpen = (event: React.MouseEvent<HTMLElement>) => {
		show_popover.set(event.currentTarget);
	};

	const handlePopoverClose = () => {
		show_popover.set(null);
	};

	const open = Boolean(show_popover.get());

	return (
		<Box width="100%" height="100%">
			<Box
				width="100%"
				height="100%"
				display="flex"
				alignItems="center"
				aria-owns={open ? 'mouse-over-popover' : undefined}
				aria-haspopup="true"
				onMouseEnter={handlePopoverOpen}
				onMouseLeave={handlePopoverClose}
			>
				{props.main}
			</Box>
			<Popover
				id="mouse-over-popover"
				sx={{
					pointerEvents: 'none'
				}}
				open={open}
				anchorEl={show_popover.get()}
				anchorOrigin={{
					vertical: 'center',
					horizontal: 'center'
				}}
				transformOrigin={{
					vertical: 'center',
					horizontal: 'center'
				}}
				onClose={handlePopoverClose}
				disableRestoreFocus
			>
				{props.pop}
			</Popover>
		</Box>
	);
};

interface PopoverButtonProps {
	button_text: string;
	pop: React.ReactNode;
}

export const PopoverButton: FC<PopoverButtonProps> = (props: PopoverButtonProps) => {
	const show_popover = new WrappedHook<HTMLElement | null>(null);
	const handlePopoverOpen = (event: React.MouseEvent<HTMLElement>) => {
		show_popover.set(event.currentTarget);
	};

	const handlePopoverClose = () => {
		show_popover.set(null);
	};

	const open = Boolean(show_popover.get());

	return (
		<Box width="100%" height="100%">
			<Chip variant="outlined" label={props.button_text} onClick={handlePopoverOpen} />
			<Popover
				id="popover"
				open={open}
				anchorEl={show_popover.get()}
				anchorOrigin={{
					vertical: 'center',
					horizontal: 'right'
				}}
				transformOrigin={{
					vertical: 'center',
					horizontal: 'left'
				}}
				onClose={handlePopoverClose}
				disableRestoreFocus
			>
				{props.pop}
			</Popover>
		</Box>
	);
};
