// global store
import { writable, type Writable } from 'svelte/store';

export const currentGroup: Writable<string> = writable();
