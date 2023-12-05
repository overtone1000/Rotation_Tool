import { sveltekit } from '@sveltejs/kit/vite';
import { defineConfig, type UserConfigExport } from 'vite';

export default defineConfig(
	({command,mode,ssrBuild})=>{
		const retval:UserConfigExport = {
			plugins: [sveltekit()]
		};

		if (command==="serve")
		{
			process.env.VITE_POST_ROOT="http://localhost:8080";
		}
		else if (command==="build")
		{
			process.env.VITE_POST_ROOT="";
		}

		return retval;
	}
);