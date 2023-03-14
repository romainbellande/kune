import { GetCurrentUserStore } from '$houdini'
import { redirect } from '@sveltejs/kit';
import type { PageLoad } from './$houdini'

export const load: PageLoad = async (event) => {
  const getCurrentUser = new GetCurrentUserStore();
   const currentUser = await getCurrentUser.fetch({ event });
   const groups = currentUser.data?.getCurrentUser.groups;

   if (groups && groups.length > 0) {
    const groupId = groups[0].id;
    throw redirect(302, `/groups/${groupId}`)
   }

    return {
        ...currentUser
    }
}
