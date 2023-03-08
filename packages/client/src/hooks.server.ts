import { SvelteKitAuth } from '@auth/sveltekit';
import Auth0Provider from '@auth/core/providers/auth0';
import { PUBLIC_AUTH0_DOMAIN, PUBLIC_AUTH0_CLIENT_ID, PUBLIC_API_URL } from '$env/static/public';
import { PRIVATE_AUTH0_CLIENT_SECRET, PRIVATE_AUTH0_SECRET } from '$env/static/private';
import { redirect } from '@sveltejs/kit';

export const handle = SvelteKitAuth({
	secret: PRIVATE_AUTH0_SECRET,
	session: {
		strategy: 'jwt'
	},
	providers: [
		//@ts-expect-error issue https://github.com/nextauthjs/next-auth/issues/6174
		Auth0Provider({
			clientId: PUBLIC_AUTH0_CLIENT_ID,
			clientSecret: PRIVATE_AUTH0_CLIENT_SECRET,
			issuer: `https://${PUBLIC_AUTH0_DOMAIN}`,
			authorization: {
				params: {
					audience: PUBLIC_API_URL,
					useRefreshTokens: true
				}
			}
		})
	],
	callbacks: {
		session: ({ session, token }) => {
			if (session?.user) {
				session.user.id = token.sub;
			}

			session.accessToken = token.access_token;

			return session;
		},
		jwt: async ({ token, account, profile }) => {
			if (account) {
				return {
					...token,
					access_token: account.access_token,
					expires_at: Math.floor(Date.now() / 1000 + (account.expires_in || 0)),
					user: token.user
				};
			} else if (Date.now() < token.expires_at * 1000 && token.access_token) {
				return token;
			}

			throw redirect(307, '/auth/signin');
		}
	}
});
