import type { Handle } from '@sveltejs/kit'
import authService from './services/auth.service';

export const handle: Handle = async ({ event, resolve }) => {
  console.log('handle client');
  // await authService.init(event.url);
  return await resolve(event)
}
