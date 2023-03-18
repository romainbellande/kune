<script lang="ts">
	import type { PageData } from './$houdini';
	import { Groupform } from '$lib/components';

	export let data: PageData;

	$: groups = data?.data?.getCurrentUser.groups;
</script>

{#if groups?.length === 0}
	<div class="flex h-full items-center justify-center">
		<Groupform />
	</div>
{:else}
	your groups
{/if}
