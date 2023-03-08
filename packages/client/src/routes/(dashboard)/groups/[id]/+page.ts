import type { PageLoad } from './$types';

export const load = (({ params }) => {
	const groupId = params.id;
	console.log('groupId :>> ', groupId);
}) satisfies PageLoad;
