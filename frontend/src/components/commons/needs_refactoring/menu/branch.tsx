import { Button, IconButton, Menu } from '@mui/material';
import React from 'react';
import { menu_style } from './autoscheda_menu';
import { Button_Props } from './button';
import { createLeaf, Leaf_Props } from './leaf';

export interface BranchBase_Props {
	key: string;
	id: string;
	leaves: Button_Props[];
}

export interface Branch_Props extends BranchBase_Props {
	label: string;
}

export interface PictureBranch_Props extends BranchBase_Props {
	icon: React.ReactNode;
}

interface Branch_State {
	anchorEl: EventTarget & HTMLButtonElement;
	open: boolean;
}

export abstract class BranchBase<T extends BranchBase_Props> extends React.Component<
	T,
	Branch_State
> {
	constructor(props: T) {
		super(props);
		console.debug('Branch instantiated', props);
		this.state = { anchorEl: null, open: false };
	}

	handleClick = (event: React.MouseEvent<HTMLButtonElement>) => {
		this.setState({ anchorEl: event.currentTarget, open: true });
	};

	handleClose = () => {
		this.setState({ anchorEl: null, open: false });
	};

	handleKeyDown = (event: React.KeyboardEvent) => {
		if (event.key === 'Tab') {
			event.preventDefault();
			this.handleClose();
		}
	};

	renderPopper = () => {
		let leaves: React.ReactNode[] = [];
		for (let prototype of this.props.leaves) {
			let fullprops: Leaf_Props = {} as Leaf_Props;
			for (let key in prototype) {
				fullprops[key] = prototype[key];
			}
			fullprops.branch = this;
			leaves.push(createLeaf(fullprops));
		}

		const handleclosefunc = (evt: any) => this.handleClose();
		const keydown = (evt: React.KeyboardEvent) => this.handleKeyDown(evt);

		/*
    return (
    <Popper key="popper" open={this.state.open} anchorEl={this.state.anchorEl} role={undefined} transition disablePortal>
        {({ TransitionProps, placement }) => (
          <Grow
            {...TransitionProps}
            style={{ transformOrigin: placement === 'bottom' ? 'center top' : 'center bottom' }}
          >
            <Paper>
              <ClickAwayListener onClickAway={handleclosefunc}>
                <MenuList autoFocusItem={this.state.open} id="menu-list-grow" onKeyDown={keydown}>
                  {leaves}
                </MenuList>
              </ClickAwayListener>
            </Paper>
          </Grow>
        )}
      </Popper>
    );
    */

		return (
			<Menu open={this.state.open} anchorEl={this.state.anchorEl} onClose={handleclosefunc}>
				{leaves}
			</Menu>
		);
	};
}

export class Branch extends BranchBase<Branch_Props> {
	render() {
		//console.debug("Creating branch",this.props);
		return (
			<div>
				<Button
					style={menu_style}
					id={this.props.id}
					aria-controls="menu"
					aria-haspopup="true"
					onClick={(evt) => this.handleClick(evt)}
				>
					{this.props.label}
				</Button>

				{this.renderPopper()}
			</div>
		);
	}
}

export class PictureBranch extends BranchBase<PictureBranch_Props> {
	render() {
		return (
			<div>
				<IconButton
					style={menu_style}
					id={this.props.id}
					aria-controls="menu"
					aria-haspopup="true"
					onClick={(evt) => this.handleClick(evt)}
				>
					{this.props.icon}
				</IconButton>

				{this.renderPopper()}
			</div>
		);
	}
}
