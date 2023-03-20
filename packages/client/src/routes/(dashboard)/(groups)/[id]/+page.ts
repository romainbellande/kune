import { GetCurrentGroupStore } from '$houdini';
import type { PageLoad } from './$types';
import { currentGroup } from '$lib/store';

export const load: PageLoad = async (event) => {
  const gidPromise = new Promise (resolve => {
    currentGroup.subscribe(gid => {
      if (gid) {
        resolve(gid);
      }
    });
  });

  await gidPromise;

  const getCurrentGroup = new GetCurrentGroupStore();
  const currentGroupResponse = await getCurrentGroup.fetch({ event });

  return {
    ...currentGroupResponse
  };
};
