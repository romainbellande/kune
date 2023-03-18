<script lang="ts">
	import { AppRail } from '@skeletonlabs/skeleton';
	import SidebarItem from './sidebar-item.svelte';
	import { currentGroup } from '$lib/store';

	interface SidebarItemProps {
		href: string;
		icon: string;
		label: string;
	}

	export const items: SidebarItemProps[] = [];

	if ($currentGroup) {
		const groupItems: SidebarItemProps[] = [
			{
				href: '/referendum',
				icon: 'mdi:people-group',
				label: 'referendum'
			}
		];

		items.push(...groupItems);
	}
</script>

<div class="hidden h-full shadow-lg lg:block">
	<AppRail>
		{#each items as item, index}
			<SidebarItem href={item.href} icon={item.icon} label={item.label} />
		{/each}
	</AppRail>
</div>
