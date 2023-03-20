import type { PageLoad } from './$types';

export const load: PageLoad = async (event) => {
	const gid = event.params.id;
	console.log('gid :>> ', gid);
};
