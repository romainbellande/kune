// See https://kit.svelte.dev/docs/types#app
// for information about these interfaces
declare global {
	namespace App {
		// interface Error {}
		// interface Locals {}
		// interface PageData {}
		// interface Platform {}
	}
}

import { DefaultSession } from '@auth/core';

declare module "@auth/core" {
  interface Session extends DefaultSession {
		user?: {
				id?: string | null;
		} & DefaultSession["user"];
  }
}

