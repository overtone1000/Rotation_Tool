import { constraint_col } from '../../needs_refactoring/displays/SegmentDisplay';
import { AssignmentSegmentType } from '../extended_types/bndata/Segmemt';
import { StagingSelectionMode } from '../staging/members/highlighting';

export const autoscheda_theme = {
	palette: {
		mode: 'dark',
		background: {
			default: '#01111c', //Main background of tables, blank page
			paper: '#1b2329' //Appbar, dialogs
		}
	}
};

export const waiting_background_color = '#00000088';
//#013254
//#1b2329

export const getAssignmentSegmentColor = (type: AssignmentSegmentType | -1) => {
	switch (type) {
		case AssignmentSegmentType.NotWorking:
			return '#2079ad'; //blue
		case AssignmentSegmentType.Off:
			return '#70b6de';
		case AssignmentSegmentType.OnCall:
			return '#efd8a4';
		case AssignmentSegmentType.Task:
			return '#905da9';
		case AssignmentSegmentType.Working: //working
			return '#ecbf58';
		case AssignmentSegmentType.Moonlighting:
			return '#2e9b63';
		case constraint_col:
			return '#ffffff'; //white
		default:
			console.debug('Unhandled type ' + type + ', rendering as red.');
			return 'red';
	}
};

export const getGhostAssignmentSegmentColor = (type: AssignmentSegmentType | -1) => {
	const basecolor = getAssignmentSegmentColor(type);
	return ColorLuminance(basecolor, -0.6);
};

//adapted from https://www.sitepoint.com/javascript-generate-lighter-darker-color/
function ColorLuminance(hex: string, lum: number) {
	// validate hex string
	hex = String(hex).replace(/[^0-9a-f]/gi, '');
	if (hex.length < 6) {
		hex = hex[0] + hex[0] + hex[1] + hex[1] + hex[2] + hex[2];
	}
	lum = lum || 0;

	// convert to decimal and change luminosity
	let rgb = '#',
		c,
		i;
	for (i = 0; i < 3; i++) {
		c = parseInt(hex.substring(i * 2, i * 2 + 2), 16);
		c = Math.round(Math.min(Math.max(0, c + c * lum), 255)).toString(16);
		rgb += ('00' + c).substring(c.length);
	}

	return rgb;
}

export const getStagingHighlightKeyframes = (highlighting: StagingSelectionMode) => {
	switch (highlighting) {
		case StagingSelectionMode.none:
		case StagingSelectionMode.primary_selected:
			return [{ borderColor: '#ff0000' }, { borderColor: '#330000' }];
		case StagingSelectionMode.secondary_selected:
			return [{ borderColor: '#0000ff' }, { borderColor: '#000033' }];
		case StagingSelectionMode.proposed:
			return [{ borderColor: '#FFFFFF' }, { borderColor: '#000000' }];
		case StagingSelectionMode.commit:
			return [{ borderColor: '#c9952c' }, { borderColor: '#705318' }];
		default:
			return [{ borderColor: 'transparent' }, { borderColor: 'transparent' }];
	}
};

export const RABackgroundColors = {
	default: {
		locked: {
			assigned: '#666666',
			unassigned: '#842a46'
		},
		unlocked: {
			unassigned: '#004400',
			assigned: '#113791'
		}
	},
	hover: {
		locked: {
			assigned: '#888888',
			unassigned: '#84485b'
		},
		unlocked: {
			unassigned: '#005500',
			assigned: '#485e91'
		}
	}
};

export const selectedStagingDateBackgroundColor = '#595959';
export const stagingCellLeftRight = '#FFFFFF';
export const stagingCellTopBottom = '#444444';

export const rc_bg_col = '#3063a5';
export const rc_bg_col_hover = '#5b789e';

export const ConstraintMemberSelectorColors = {
	default_color: '#003052',
	mouseover_color: '#0062a8',
	selected_color: '#000d85'
};

export const sizing = {
	keyed_row_height: '35px'
};
