<script lang="ts">
	import { Breadcrumb, BreadcrumbItem } from 'flowbite-svelte';
	import { page } from '$app/stores';
	import Sidebar from '@app/components/sidebar.svelte';
	import Navbar from '@app/components/navbar.svelte';
	import type { PageData } from './$houdini';

	export let data: PageData;

	$: title = $page.data.title;

	$: ({ GetCurrentUser } = data);
	$: groups = $GetCurrentUser.data?.getCurrentUser?.groups;

	$: breadcrumbs = $page.url.pathname
		.split('/')
		.map((pathname) => ({
			home: !pathname,
			text: pathname || 'home',
			url: pathname ? $page.url.pathname.split(pathname)[0] + pathname : '/'
		}))
		.filter(({ url }, index) => !(url === '/' && index > 0));
</script>

{#if $page.data.session}
	<div class="flex min-h-screen">
		<Sidebar />
		<div class="flex flex-grow flex-col">
			<Navbar {groups} />
			<div class="flex-grow bg-slate-50 p-4">
				<Breadcrumb aria-label="Default breadcrumb example">
					{#each breadcrumbs as breadcrumb}
						<BreadcrumbItem home={breadcrumb.home} href={breadcrumb.url}
							>{breadcrumb.text}</BreadcrumbItem
						>
					{/each}
				</Breadcrumb>
				<div class="pt-8 text-slate-800">
					{#if title}
						<h2 class="pb-16 text-xl">{title}</h2>
					{/if}
					<slot />
				</div>
			</div>
		</div>
	</div>
{:else}
	<h1>Access Denied</h1>
	<p>
		<a href="/auth/signin"> You must be signed in to view this page </a>
	</p>
{/if}
