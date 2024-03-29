// See https://kit.svelte.dev/docs/types#app
// for information about these interfaces
declare global {
	namespace App {
		// interface Error {}
		// interface Locals {}
		// interface PageData {}
		// interface Platform {}
		interface Session {
			accessToken?: string;
		}
	}
}

import { DefaultSession } from '@auth/core';

declare module '@auth/core' {
	interface Session extends DefaultSession {
		user?: {
			id?: string | null;
		} & DefaultSession['user'];
		accessToken?: string;
	}

	export interface JWT extends Record<string, unknown>, DefaultJWT {
		expires_at: number;
	}
}

// Currently `createEventDispatcher` is not 100% typesafe. Until it gets merged, you can use the following code snippet to improve the type definition.
// There is an open PR that improves the type definitions: https://github.com/sveltejs/svelte/pull/7224

import 'svelte';

declare module 'svelte' {
	type UnionToIntersection<U> = (U extends any ? (k: U) => void : never) extends (
		k: infer I
	) => void
		? I
		: never;

	// eslint-disable-next-line @typescript-eslint/ban-types
	type ExtractObjectValues<Object extends Record<any, any>> = Object[keyof Object];

	type ConstructDispatchFunction<
		EventMap extends Record<string, any>,
		EventKey extends keyof EventMap
	> = EventMap[EventKey] extends never | null
		? (type: EventKey) => void
		: null extends EventMap[EventKey]
		? (type: EventKey, detail?: EventMap[EventKey]) => void
		: (type: EventKey, detail: EventMap[EventKey]) => void;

	type CreateDispatchFunctionMap<EventMap> = {
		[Key in keyof EventMap]: ConstructDispatchFunction<EventMap, Key>;
	};

	type EventDispatcher<EventMap extends Record<string, any>> = UnionToIntersection<
		ExtractObjectValues<CreateDispatchFunctionMap<EventMap>>
	>;

	/**
	 * @example
	 * ```ts
	 * const dispatch = createEventDispatcher<
	 * 	click: number // define the type of the event's `detail`
	 * 	loaded: never // use `never` if the event should not contain any payload
	 * 	'custom-event': string | null // use an union type and add `null` if the payload is optional
	 * >()
	 * ```
	 */
	export declare function createEventDispatcher<
		EventMap extends Record<string, any> = any
	>(): EventDispatcher<EventMap>;
}
