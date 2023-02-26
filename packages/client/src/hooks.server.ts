import { SvelteKitAuth } from "@auth/sveltekit"
import Auth0Provider from "@auth/core/providers/auth0"
import { PUBLIC_AUTH0_DOMAIN, PUBLIC_AUTH0_CLIENT_ID, PUBLIC_API_URL } from '$env/static/public';
import { PRIVATE_AUTH0_CLIENT_SECRET, PRIVATE_AUTH0_SECRET } from '$env/static/private';

export const handle = SvelteKitAuth({
    secret: PRIVATE_AUTH0_SECRET,
    session: {
        strategy: 'jwt',

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
                },
            },
         })
    ],
    callbacks: {

        session: ({ session, token }) => {
            if (session?.user) {
                session.user.id = token.sub;
            }

            session.accessToken = token.accessToken;

            return session;
        },
        jwt: async ({ token, account, profile }) => {
            if (account && profile) {
                return {
                    ...token,
                    accessToken: account.access_token
                };
            }

            return token;
        }
    }
})

// import { setSession } from '$houdini'
// import type { Handle } from '@sveltejs/kit'
// import { token } from './store';
// import { get } from 'svelte/store';

// export const handle: Handle = async ({ event, resolve }) => {

//     // get the user information however you want
//     console.log('pong pong', event.cookies.get(''));
//     const tokenValue = get(token);

//     // set the session information for this event
//     setSession(event, { token: tokenValue })

//     // pass the event onto the default handle
//     return await resolve(event)
// }
