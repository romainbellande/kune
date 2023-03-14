import type { LayoutLoad } from "./$types";
import { currentGroup } from '$lib/store';

export const load = (({ params }) => {
  const groupId = params.id;
  currentGroup.set(groupId);
}) satisfies LayoutLoad;
