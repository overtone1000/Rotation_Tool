export function findGetParameter(parameterName: string) {
	const params = window.location.search.substr(1).split('&');
	for (const i in params) {
		const param = params[i];
		let tmp = param.split('=');
		console.debug('Comparing:', tmp[0], parameterName, tmp[0] === parameterName);
		if (tmp[0] === parameterName) {
			console.debug('Returning', tmp[1]);
			return tmp[1];
		}
	}

	console.debug('Parameter ' + parameterName + ' not found.');
	return undefined;
}

export function findCookie(cookieName: string) {
	const cookies = document.cookie.split('; ');
	for (const i in cookies) {
		const cookie = cookies[i];
		let tmp = cookie.split('=');
		//console.debug("Comparing:",tmp[0],parameterName,tmp[0] === parameterName);
		if (tmp[0] === cookieName) {
			console.debug('Returning', tmp[1]);
			return tmp[1];
		}
	}

	console.debug('Cookie ' + cookieName + ' not found.');
	return undefined;
}

export enum CursorRelevantState {
	WaitingOnServerResponse,
	HoverOverInteractable
}
const statuses = {
	[CursorRelevantState.WaitingOnServerResponse]: false,
	[CursorRelevantState.HoverOverInteractable]: false
};
export const setCursorRelevantState = (interaction: CursorRelevantState, state: boolean) => {
	statuses[interaction] = state;
	if (statuses[CursorRelevantState.WaitingOnServerResponse]) {
		window.document.body.style.cursor = 'wait';
	} else if (statuses[CursorRelevantState.HoverOverInteractable]) {
		window.document.body.style.cursor = 'pointer';
	} else {
		window.document.body.style.cursor = 'default';
	}
};
