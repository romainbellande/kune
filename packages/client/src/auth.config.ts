import { PUBLIC_OAUTH_DOMAIN, PUBLIC_OAUTH_CLIENT_ID } from '$env/static/public';
import type { Auth0ClientOptions } from '@auth0/auth0-spa-js';

export const config: Auth0ClientOptions = {
  domain: PUBLIC_OAUTH_DOMAIN,
  clientId: PUBLIC_OAUTH_CLIENT_ID,
  authorizationParams: {
    redirect_uri: 'http://localhost:5137',
    audience: 'http://localhost:3000'
  },
  useRefreshTokens: true,
};
