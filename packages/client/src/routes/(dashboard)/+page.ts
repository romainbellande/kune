import authService from '../../services/auth.service';
import type { PageLoad } from './$types';

export const load = (({ url }) => {
  authService.init(url);
}) satisfies PageLoad;

export const ssr = false;

