<script lang="ts">
	import { createEventDispatcher } from 'svelte';
	import type { ChangeEventHandler } from 'svelte/elements';

	interface SelectEvent {
		change: string;
	}
	const dispatch = createEventDispatcher<SelectEvent>();

	export let customClass = '';

	const onChange: ChangeEventHandler<HTMLSelectElement> = (event) => {
		dispatch('change', event.currentTarget.value);
	};
</script>

<select class="select {customClass}" on:change={onChange}>
	<slot />
</select>
