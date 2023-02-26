import { PUBLIC_OAUTH_DOMAIN, PUBLIC_OAUTH_CLIENT_ID, PUBLIC_OAUTH_REDIRECT_URI, PUBLIC_API_URL } from '$env/static/public';
import type { Auth0ClientOptions } from '@auth0/auth0-spa-js';

export const config: Auth0ClientOptions = {
  domain: PUBLIC_OAUTH_DOMAIN,
  clientId: PUBLIC_OAUTH_CLIENT_ID,
  authorizationParams: {
    redirect_uri: PUBLIC_OAUTH_REDIRECT_URI,
    audience: PUBLIC_API_URL
  },
  useRefreshTokens: true,
};
