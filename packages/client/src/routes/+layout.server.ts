import type { LayoutServerLoad } from "./$types"
import { setSession } from '$houdini'
import { redirect } from "@sveltejs/kit";

export const load: LayoutServerLoad = async (event) => {
  const session = await event.locals.getSession();
  setSession(event, session);

  if (!session) {
    throw redirect(307, '/auth/signin');
  }

  return {
    session
  }
}
