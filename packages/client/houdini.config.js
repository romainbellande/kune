// import { PUBLIC_API_URL } from '$env/static/public';
/// <references types="houdini-svelte">

/** @type {import('houdini').ConfigFile} */
const config = {
	cacheBufferSize: 0,
	watchSchema: {
		url: (env) => `${env.PUBLIC_API_URL}/graphql`
	},
	plugins: {
		'houdini-svelte': {}
	},
	scalars: {
		UUID: {
			type: 'string'
		}
	},
	defaultCachePolicy: 'NetworkOnly',
	defaultLifetime: 0
};

export default config;
