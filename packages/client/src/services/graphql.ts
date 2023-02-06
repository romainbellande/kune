import { ApolloClient, InMemoryCache } from '@apollo/client';
import { PUBLIC_GRAPHQL_URI } from '$env/static/public';
import { token } from '../store';
import { get } from 'svelte/store';

export const createClient = () => {
  const currentToken = get(token);

  return new ApolloClient({
    uri: PUBLIC_GRAPHQL_URI,
    cache: new InMemoryCache(),
    headers: {
      'Content-Type': 'application/json',
      'Authorization': `bearer ${currentToken}`
    }
  });
}
