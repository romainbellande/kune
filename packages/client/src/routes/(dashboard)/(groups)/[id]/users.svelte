<script lang="ts">
	import Icon from '@iconify/svelte';
	import { Paginator } from '@skeletonlabs/skeleton';
	import type { PaginationSettings } from '@skeletonlabs/skeleton/components/Paginator/types';
	import { writable, type Writable } from 'svelte/store';

	interface User {
		name: string;
		role: string;
	}

	const users: User[] = [
		{
			name: 'John Doe',
			role: 'admin'
		},
		{
			name: 'Jane Doe',
			role: 'member'
		}
	];

	let page: PaginationSettings = {
		offset: 0,
		limit: 20,
		size: users.length,
		amounts: [20]
	};
</script>

<div class="card p-4 shadow-lg">
	<header class="flex-start card-header flex">
		<h4 class="capitalize-first text-center text-2xl font-medium">
			<span>users</span>
			<button type="button" class="bg-initial btn-icon">
				<Icon icon="mdi:user-add" class="text-primary-500" width={24} style="min-width: 24px;" />
			</button>
		</h4>
	</header>
	<section class="p-4 space-y-4">
		<!-- Responsive Container (recommended) -->
		<div class="table-container">
			<!-- Native Table Element -->
			<table class="table-hover table">
				<thead>
					<tr>
						<th>name</th>
						<th>role</th>
						<th>actions</th>
					</tr>
				</thead>
				<tbody>
					{#each users as user, i}
						<tr>
							<td>{user.name}</td>
							<td><span class="badge variant-filled">{user.role}</span></td>
							<td>
								<button type="button" class="bg-initial btn-icon">
									<Icon
										icon="mdi:trash-can-outline"
										class="text-error-500"
										width={24}
										style="min-width: 24px;"
									/>
								</button>
								<button type="button" class="bg-initial btn-icon">
									<Icon
										icon="material-symbols:arrow-right-alt-rounded"
										class="text-primary-500"
										width={24}
										style="min-width: 24px;"
									/>
								</button>
							</td>
						</tr>
					{/each}
				</tbody>
			</table>
		</div>
		<Paginator bind:settings={page} />
	</section>
</div>
