import { Button } from '@mui/material';
import React from 'react';
import { AutoschedaState } from '../autoscheda_core';
import { menu_style } from './autoscheda_menu';

export interface Button_Props {
	key: string;
	label: string;
	app: AutoschedaState;
	displayfunct: (props: AutoschedaState) => void;
}

export const createButton = function (props: Button_Props): React.ReactNode {
	//console.debug("Creating button " + props);
	return (
		<Button
			style={menu_style}
			id={props.key}
			key={props.key}
			onClick={(evt) => {
				props.displayfunct(props.app);
			}}
		>
			{props.label}
		</Button>
	);
};
