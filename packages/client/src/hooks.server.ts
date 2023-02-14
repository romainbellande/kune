import { setSession } from '$houdini'
import type { Handle } from '@sveltejs/kit'
import { token } from './store';
import { get } from 'svelte/store';

export const handle: Handle = async ({ event, resolve }) => {
    // get the user information however you want
    console.log('pong')
    const tokenValue = get(token);

    // set the session information for this event
    setSession(event, { token: tokenValue })

    // pass the event onto the default handle
    return await resolve(event)
}
