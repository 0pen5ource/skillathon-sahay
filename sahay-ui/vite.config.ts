import { sveltekit } from '@sveltejs/kit/vite';
import { defineConfig } from 'vite';

export default defineConfig({
	plugins: [sveltekit()],
	server: {
		proxy: {
			'/api': 'https://sahaay.xiv.in',
			'/registry': 'https://sahaay.xiv.in'
		}
	}
});
