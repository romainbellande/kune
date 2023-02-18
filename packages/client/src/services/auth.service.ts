import { Auth0Client, createAuth0Client, User } from '@auth0/auth0-spa-js';
import { auth0User, isAuthenticated, token  } from '@app/store';
import { config } from '@app/auth.config';

function createClient(): Promise<Auth0Client> {
  return createAuth0Client(config);
}

async function loginWithRedirect(client: Auth0Client) {
  try {
    await client.loginWithRedirect();

  } catch(e) {
    console.error(e);
  }
}

function logout(client: Auth0Client) {
  return client.logout();
}

async function init(url: URL) {
    const client = await createClient();

    if (urlIsRedirectCallback(url)) {
      console.log('redirect');
      await handleRedirect(client);
    }

    const isAuth = await client.isAuthenticated();

    if (!isAuth) {
      await loginWithRedirect(client);
    } else {
      await setUser(client);
      const newToken =await client.getTokenSilently();
      token.set(newToken);
      isAuthenticated.set(true);
    }
}

const setUser = async (client: Auth0Client) => {
    const currentUser: User | undefined = await client.getUser();

    if (currentUser) {
      auth0User.set(currentUser);
    }
}

const urlIsRedirectCallback = (url: URL): boolean => {
  const { search } = url;
  return search.includes("code=") && search.includes("state=");
};

async function handleRedirect(client: Auth0Client) {
    await client.handleRedirectCallback();
    window.history.replaceState({}, document.title, "/");
}

export default {
  createClient,
  loginWithRedirect,
  handleRedirect,
  logout,
  init,
};
