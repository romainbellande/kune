<script lang="ts">
	import { page } from '$app/stores';
	import BreadcrumbItem from './breadcrumb-item.svelte';

	$: breadcrumbs = $page.url.pathname
		.split('/')
		.map((pathname) => ({
			text: pathname || 'home',
			url: pathname ? $page.url.pathname.split(pathname)[0] + pathname : '/'
		}))
		.filter(({ url }, index) => !(url === '/' && index > 0));
</script>

{#if breadcrumbs.length > 1}
	<div class="m-4 flex max-w-max justify-center p-4 text-token">
		<ol class="breadcrumb ">
			{#each breadcrumbs as breadcrumb}
				<BreadcrumbItem href={breadcrumb.url}>{breadcrumb.text}</BreadcrumbItem>
			{/each}
		</ol>
	</div>
{/if}
