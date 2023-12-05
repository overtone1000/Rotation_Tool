import { Button, Grid } from '@mui/material';
import React, { FC } from 'react';
import { WrappedHook } from '../../../react/WrappedHook';
import { Interaction } from '../staging';
import { ConstraintFormComponentProps } from './constraint_form_components';

export interface RevertProps {
	parent_props: ConstraintFormComponentProps;
	parent_active_element: WrappedHook<any>;
}

export const Revert: FC<RevertProps> = (props: RevertProps) => {
	/*
    let initial_details_clone = useMemo(
        ()=>{
            console.debug("useMemo called",props.parent_props.rendered_constraint);
            return props.parent_props.rendered_constraint.getDetailsClone();   
        }
        ,[props.parent_props.rendered_constraint]
    );
    */

	const revert = () => {
		const all_entailed = props.parent_props.rendered_constraint.getEntailedAssignables(); //First, get all the entailed assignables. This will will change after update.
		props.parent_props.rendered_constraint.clearProposedDetails();
		props.parent_active_element.set(null);
		props.parent_props.interaction_handler(Interaction.releaseFocusedSelection);

		props.parent_props.rendered_constraint.getEntailedAssignables().forEach((rai) => {
			all_entailed.add(rai);
		});

		all_entailed.forEach((rai) => {
			const ra = props.parent_props.staging.rendered_assignables[rai];
			ra.clearProposedConstraints();
		});
	};

	return (
		<Button
			style={{ width: '100%', marginTop: '15px' }}
			aria-label="discard"
			color="secondary"
			variant="contained"
			onClick={revert}
			disabled={!props.parent_props.rendered_constraint.hasProposedDetails()}
		>
			Undo Changes
		</Button>
	);

	/*
    return (
    <Grid
        sx={{width:"100%", marginTop:"15px"}}
        container
        direction="row"
        justifyContent="space-around"
        alignItems="center"
    >
        <Grid item xs="auto">
            <Button aria-label="discard" color="secondary" variant="contained" onClick={revert} disabled={!props.parent_props.rendered_constraint.hasProposedDetails()}>
                Undo Changes
            </Button>
        </Grid>
    </Grid>
    );
    */
};
