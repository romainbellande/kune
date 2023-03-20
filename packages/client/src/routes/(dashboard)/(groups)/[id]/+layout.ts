import type { LayoutLoad } from './$types';
import { currentGroup } from '$lib/store';

export const load = (({ params }) => {
	const gid = params.id;
	console.log('gid server :>> ', gid);
	currentGroup.set(gid);

	return {
		gid
	};
}) satisfies LayoutLoad;
