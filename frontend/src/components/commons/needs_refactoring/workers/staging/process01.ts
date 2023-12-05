onmessage = function (e) {
	const workerResult = 'Hello!';
	postMessage(workerResult);
};
