<script lang="ts">
  import { AppShell } from '@skeletonlabs/skeleton';
  import { Sidebar, Header, Breadcrumb } from '$lib/components';
  import type { PageData } from './$houdini';

  export let data: PageData;
  $: ({ GetCurrentUser } = data);
	$: groups = $GetCurrentUser.data?.getCurrentUser?.groups;
</script>

<AppShell>
	<svelte:fragment slot="header">
    <Header {groups} />
  </svelte:fragment>
	<svelte:fragment slot="sidebarLeft">
    <Sidebar />
  </svelte:fragment>
	<svelte:fragment slot="pageHeader">
    <Breadcrumb />
  </svelte:fragment>
  <slot />
</AppShell>
