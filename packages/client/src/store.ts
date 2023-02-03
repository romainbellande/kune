import type { Auth0Client, User } from "@auth0/auth0-spa-js";
import { writable, derived } from "svelte/store";
import authService from './services/auth.service';

export const isAuthenticated = writable(false);
export const token = writable<string>();
export const user = writable<User | undefined>();
export const popupOpen = writable(false);
export const error = writable();

export const tasks = writable<any[]>([]);

export const user_tasks = derived([tasks, user], ([$tasks, $user]) => {
  let logged_in_user_tasks: any[] = [];

  if ($user && $user.email) {
    logged_in_user_tasks = $tasks.filter((task) => task.user === $user.email);
  }

  return logged_in_user_tasks;
});
