import { load_GetCurrentGroup } from '$houdini';
import type { PageLoad } from './$types';

export const load: PageLoad = async (event) => {
	const currentGroupResponse = await load_GetCurrentGroup({ event });

	return {
		...currentGroupResponse
	};
};
