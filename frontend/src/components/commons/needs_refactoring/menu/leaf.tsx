import React from 'react';
import { BranchBase, BranchBase_Props } from './branch';
import { Button_Props } from './button';

import { MenuItem } from '@mui/material';
import { menu_style } from './autoscheda_menu';

export interface Leaf_Props extends Button_Props {
	branch: BranchBase<BranchBase_Props>;
}

export const createLeaf = function (props: Leaf_Props): React.ReactNode {
	return (
		<MenuItem
			style={menu_style}
			key={props.key}
			id={props.key}
			aria-controls="simple-menu"
			aria-haspopup="true"
			onClick={(evt) => {
				if (props.branch !== null) {
					props.branch.handleClose();
				}
				console.debug('Leaf ' + props.label + ' pushed.');

				props.displayfunct(props.app);
			}}
		>
			{props.label}
		</MenuItem>
	);
};
