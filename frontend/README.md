SvelteKit: https://kit.svelte.dev/docs
Svelte Material UI: https://sveltematerialui.com/SVELTEKIT.md

For typography, follow the scss import instructions in SMUI and then rerun the `prepare` script (no need to do the others, prepare just calls those two!)

For icons, can just search `material-icons` (see https://mui.com/material-ui/material-icons/) and change from camel-case to underscores. For example `CreditScore` becomes `credit_score`.

Modifications made:
- Add the following to `svelte.config.js` change the base directory and output build to the Java project:
```
	kit: {
		adapter: adapter({
			pages: '../../backend/src/main/resources/static/svelte',
			assets: '../../backend/src/main/resources/static/svelte',
			fallback: null,
			precompress: false,
			strict: true
		}),
		paths: {
            base: "/svelte",
        }
	}
```
- Change hrefs in app.html to be relative to the index.html directory:
    `href="/smui-dark.css"` becomes `href="./smui-dark.css"`