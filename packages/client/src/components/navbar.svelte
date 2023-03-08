<script lang="ts">
	import { Navbar, Button, Dropdown, DropdownItem, Chevron } from 'flowbite-svelte';
	import Icon from '@iconify/svelte';
	import type { GetCurrentUser$result } from '$houdini';
	import { goto } from '$app/navigation';

	export let groups: GetCurrentUser$result['getCurrentUser']['groups'] | undefined;
</script>

<Navbar
	navClass="relative py-3 bg-white"
	navDivClass="mx-auto flex flex-wrap space-x-4 items-center px-4 justify-between"
	fluid={false}
	color="gray"
>
	{#if groups && groups.length > 0}
		<div>
			<Button outline><Chevron>Select a group</Chevron></Button>
			<Dropdown>
				{#each groups as group}
					<DropdownItem on:click={() => goto(`/groups/${group.id}`)} key={group.id}
						>{group.name}</DropdownItem
					>
				{/each}
			</Dropdown>
		</div>
	{/if}
	<Button outline on:click={() => goto(`/groups/create`)}>
		Create a group
		<Icon class="ml-2" icon="material-symbols:add-circle-outline" height={20} width={20} />
	</Button>
</Navbar>
