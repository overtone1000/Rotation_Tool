import { Avatar, Box } from '@mui/material';
import React from 'react';
import {
	logout,
	rawnavigate,
	requestAssignmentTypes,
	requestGlobalRequests,
	requestIndividualRequests,
	requestMySchedule,
	requestResponsibility,
	requestScheduleByAssignment,
	requestScheduleByWorker,
	requestScheduleTemplates,
	requestSolver,
	requestStaging,
	requestTrackingAdjustments,
	requestTrackingTotals,
	requestWebUsers,
	requestWorkerRoster
} from '../ajax/commands_generic';
import { AutoschedaState } from '../autoscheda_core';
import { findCookie } from '../commons/DOMfunctions';
import { Branch, Branch_Props, PictureBranch, PictureBranch_Props } from './branch';
import { Button_Props, createButton } from './button';

export const createAutoschedaMenu = function (app: AutoschedaState): React.ReactNode[] {
	let retval = [
		createScheduleBranch(app),
		createTypesBranch(app),
		createWorkersBranch(app),
		createRequestBranch(app),
		createSchedulingBranch(app),
		createTrackingBranch(app),
		createWebUsersButton(app),
		<Box key="spacer" style={{ flexGrow: 1 }}></Box>,
		createUserBranch(app)
	];

	return retval;
};

export const menu_style: React.CSSProperties = { fontSize: 14 };

const createUserBranch = function (app: AutoschedaState): React.ReactNode {
	const settings_url: string = '/user_settings';
	const props: PictureBranch_Props = {
		key: 'userbranch',
		id: undefined,
		icon: <Avatar src={findCookie('image')} />, //(<AccountCircleIcon/>),
		leaves: [
			{
				key: 'settings',
				label: 'Settings',
				app: app,
				displayfunct: () => {
					rawnavigate(app, settings_url);
				}
			},
			{
				key: 'logout',
				label: 'Logout',
				app: app,
				displayfunct: () => logout(app)
			}
		]
	};
	props.id = props.key;
	return React.createElement(PictureBranch, props, null);

	/*
  return (
  <form method="post" action={logouturl}>
    {getCSRFFormInput()}
    <Button type="submit"> Logout </Button>
  </form>
  );
  */
};

const createScheduleBranch = function (app: AutoschedaState): React.ReactNode {
	const props: Branch_Props = {
		key: 'viewschedulebranch',
		id: undefined,
		label: 'View Schedule',
		leaves: [
			{
				key: 'viewmyschedule',
				label: 'My Schedule',
				app: app,
				displayfunct: () => requestMySchedule(app)
			},
			{
				key: 'viewschedulebyworkerbutton',
				label: 'Schedule By Worker',
				app: app,
				displayfunct: () => requestScheduleByWorker(app)
			},
			{
				key: 'viewschedulebyassignmentbutton',
				label: 'Schedule By Assignment',
				app: app,
				displayfunct: () => requestScheduleByAssignment(app)
			}
		]
	};
	props.id = props.key;
	return React.createElement(Branch, props, null);
};

const createTypesBranch = function (app: AutoschedaState): React.ReactNode {
	const props: Branch_Props = {
		key: 'viewtypesbranch',
		id: undefined,
		label: 'Types',
		leaves: [
			{
				key: 'viewassignmenttypes',
				label: 'Assignment Types',
				app: app,
				displayfunct: () => requestAssignmentTypes(app)
			},
			{
				key: 'viewscheduletemplatetypes',
				label: 'Schedule Templates',
				app: app,
				displayfunct: () => requestScheduleTemplates(app)
			}
		]
	};
	props.id = props.key;
	return React.createElement(Branch, props, null);
};

const createWorkersBranch = function (app: AutoschedaState): React.ReactNode {
	const props: Branch_Props = {
		key: 'viewworkerinfo',
		id: undefined,
		label: 'Workers',
		leaves: [
			{
				key: 'viewroster',
				label: 'Roster',
				app: app,
				displayfunct: () => requestWorkerRoster(app)
			},
			{
				key: 'viewresponsibility',
				label: 'Responsibility',
				app: app,
				displayfunct: () => requestResponsibility(app)
			}
		]
	};
	props.id = props.key;
	return React.createElement(Branch, props, null);
};

const createRequestBranch = function (app: AutoschedaState): React.ReactNode {
	const props: Branch_Props = {
		key: 'requestbranch',
		id: undefined,
		label: 'Requests',
		leaves: [
			{
				key: 'globalrequestsbutton',
				label: 'Global',
				app: app,
				displayfunct: () => requestGlobalRequests(app)
			},
			{
				key: 'individualrequestsbutton',
				label: 'Individual',
				app: app,
				displayfunct: () => requestIndividualRequests(app)
			}
		]
	};
	props.id = props.key;
	return React.createElement(Branch, props, null);
};

const createSchedulingBranch = function (app: AutoschedaState): React.ReactNode {
	const props: Branch_Props = {
		key: 'schedulingbranch',
		id: undefined,
		label: 'Scheduling',
		leaves: [
			{
				key: 'stagingbutton',
				label: 'Staging',
				app: app,
				displayfunct: () => requestStaging(app)
			},
			{
				key: 'solutionbutton',
				label: 'Solve',
				app: app,
				displayfunct: () => requestSolver(app)
			}
		]
	};
	props.id = props.key;
	return React.createElement(Branch, props, null);
};

const createTrackingBranch = function (app: AutoschedaState): React.ReactNode {
	/*
  const createTotalsDisplay=function(app:AutoschedaState):void
  {
    app.setStaticDisplay(
      <div>
        Tracking Totals
      </div>
    );
  };

  const createAdjustmentsDisplay=function(app:AutoschedaState):void
  {
    app.setStaticDisplay(
      <div>
        Tracking Adjustments
      </div>
    );
  };
  */

	const props: Branch_Props = {
		key: 'trackingbranch',
		id: undefined,
		label: 'Tracking',
		leaves: [
			{
				key: 'trackingtotalsbutton',
				label: 'Totals',
				app: app,
				displayfunct: () => {
					requestTrackingTotals(app);
				}
			},
			{
				key: 'trackingadjustmentsbutton',
				label: 'Adjustments',
				app: app,
				displayfunct: () => {
					requestTrackingAdjustments(app);
				}
			}
		]
	};
	props.id = props.key;
	return React.createElement(Branch, props, null);
};

const createWebUsersButton = function (app: AutoschedaState): React.ReactNode {
	const props: Button_Props = {
		key: 'webusersbutton',
		label: 'Web Users',
		app: app,
		displayfunct: () => {
			requestWebUsers(app);
		}
	};
	return createButton(props);
};
