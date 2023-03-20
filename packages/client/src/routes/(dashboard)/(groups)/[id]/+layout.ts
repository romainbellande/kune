import type { LayoutLoad } from './$types';
import { currentGroup } from '$lib/store';

export const load = (async ({ params }) => {
	const gid = params.id;
	console.log('gid server :>> ', gid);
	currentGroup.set(gid);

	const gidPromise = new Promise((resolve) => {
		currentGroup.subscribe((gid) => {
			if (gid) {
				resolve(gid);
			}
		});
	});

	await gidPromise;

	return {
		gid
	};
}) satisfies LayoutLoad;
