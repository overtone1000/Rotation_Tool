import adapter from '@sveltejs/adapter-static';
import { vitePreprocess } from '@sveltejs/kit/vite';

///** @type {import('@sveltejs/kit').Config} */
const config = {
	// Consult https://kit.svelte.dev/docs/integrations#preprocessors
	// for more information about preprocessors
	preprocess: vitePreprocess(),

	kit: {
		adapter: adapter({
			// default options are shown. On some platforms
			// these options are set automatically — see below
			pages: '../../backend/src/main/resources/static/svelte',
			assets: '../../backend/src/main/resources/static/svelte',
			fallback: null,
			precompress: false,
			strict: true
		}),
		paths: {
			base: '/svelte'
		}
		//alias: {
		//	"@@migration" : "../../frontend/src/ts/"
		//}
	}
};

export default config;
