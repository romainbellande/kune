<script lang="ts">
	import { AppBar, Avatar } from '@skeletonlabs/skeleton';
	import Icon from '@iconify/svelte';
	import logo from '$lib/assets/logo.png';
	import { Select } from '$lib/components';
	import type { GetCurrentUser$result } from '$houdini';
	import { goto } from '$app/navigation';

	export let groups: GetCurrentUser$result['getCurrentUser']['groups'] = [];

	const onGroupChange = ({ detail }: CustomEvent<string>) => {
		const groupId = detail;
		goto(`/${groupId}`);
	}
</script>

<AppBar
	gridColumns="grid-cols-3"
	slotDefault="place-self-center"
	slotTrail="place-content-end"
	padding="p-2"
	shadow="shadow-xl"
>
	<svelte:fragment slot="lead">
		<div>
			<button type="button" class="bg-initial btn-icon lg:hidden">
				<Icon icon="ci:hamburger-md" width={24} style="min-width: 24px;" />
			</button>
			{#if groups.length > 0}
			<Select customClass="ml-[70px]" on:change={onGroupChange}>
				{#each groups as group}
					<option value={group.id}>{group.name}</option>
				{/each}
			</Select>
			{/if}
		</div>
	</svelte:fragment>

	<img class="h-16 w-16 lg:hidden" alt="Kune logo" src={logo} />
  <h1 class="capitalize hidden lg:block">kune</h1>

	<svelte:fragment slot="trail">
		<Avatar initials="JD" background="bg-primary-500" width="w-12" />
	</svelte:fragment>
</AppBar>
