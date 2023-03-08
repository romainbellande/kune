import type { PageLoad } from './$types';

export const load = (({ params }) => {
	return {
		title: 'Create new group'
	};
}) satisfies PageLoad;
