<script lang="ts">
	import type { PageData } from './$houdini';
	import {
		Table,
		TableBody,
		TableBodyCell,
		TableBodyRow,
		TableHead,
		TableHeadCell,
		Button
	} from 'flowbite-svelte';
	import Icon from '@iconify/svelte';
	import { goto } from '$app/navigation';
	export let data: PageData;

	$: ({ GetCurrentUser } = data);
	$: groups = $GetCurrentUser.data?.getCurrentUser?.groups;
	console.log('groups :>> ', groups);
</script>

<div>
	<Table>
		<TableHead>
			<TableHeadCell>Name</TableHeadCell>
			<TableHeadCell>Users</TableHeadCell>
			<TableHeadCell>Actions</TableHeadCell>
		</TableHead>
		<TableBody>
			{#each groups || [] as group}
				<TableBodyRow>
					<TableBodyCell>{group.name}</TableBodyCell>
					<TableBodyCell>{group.users.length}</TableBodyCell>
					<TableBodyCell>
						<Button pill={true} class="!p-2" on:click={() => goto(`/groups/${group.id}`)}>
							<Icon icon="material-symbols:arrow-forward-rounded" height={20} width={20} />
						</Button>
					</TableBodyCell>
				</TableBodyRow>
			{/each}
		</TableBody>
	</Table>
</div>
