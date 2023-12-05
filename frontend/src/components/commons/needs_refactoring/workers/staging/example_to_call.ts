const worker = new Worker(new URL('../../workers/staging/process01.ts', import.meta.url), {
	type: 'module'
});
worker.onmessage = (e: MessageEvent<any>) => {
	console.debug('Worker onmessage', e);
};
