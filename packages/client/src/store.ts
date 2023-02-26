import type { User } from "@auth0/auth0-spa-js";
import { writable } from "svelte/store";
import type { GetUserByExternalId$result } from '$houdini';

export const isAuthenticated = writable(false);
export const token = writable<string>();
export const auth0User = writable<User | undefined>();
export const popupOpen = writable(false);
export const error = writable();
export const user = writable<GetUserByExternalId$result | null>();
