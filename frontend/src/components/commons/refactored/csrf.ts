const csrf_token_cookie = 'CSRF-TOKEN-COOKIE';
export const csrf_token_parameter = 'CSRF-TOKEN-PARAMETER';
export const csrf_token_header = 'CSRF-TOKEN-HEADER';

export const getCSRFFormInput = () => {
	/*
  return (
  <Input
    type="hidden"
    name={csrf_token_parameter}
    value={getCSRFToken()}
  />
  );
  */
	console.error('Deprecated.');
};

export const getDOMCSRFFormInput = () => {
	console.debug('Creating CSRF input');
	const retval = document.createElement('input');
	retval.type = 'hidden';
	retval.name = csrf_token_parameter;
	retval.value = getCSRFToken();
	return retval;
};

export const getCSRFToken = () => {
	return getCookie(csrf_token_cookie);
};

function getCookie(cname: string) {
	var name = cname + '=';
	var decodedCookie = decodeURIComponent(document.cookie);
	var ca = decodedCookie.split(';');
	for (var i = 0; i < ca.length; i++) {
		var c = ca[i];
		while (c.charAt(0) == ' ') {
			c = c.substring(1);
		}
		if (c.indexOf(name) == 0) {
			return c.substring(name.length, c.length);
		}
	}
	return '';
}
