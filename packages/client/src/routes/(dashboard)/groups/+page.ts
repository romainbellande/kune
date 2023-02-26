import { GetCurrentUserStore } from '$houdini';
import type { LoadEvent } from '@sveltejs/kit';
import type { PageLoad } from './$types';

export const load = (async (event) => {
  const getCurrentUser = new GetCurrentUserStore();
  const { data: currentUser } = await getCurrentUser.fetch({ event });
  console.log('currentUser :>> ', currentUser);
  return currentUser;
}) satisfies PageLoad;
