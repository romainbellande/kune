<script lang="ts">
  import { Breadcrumb, BreadcrumbItem } from 'flowbite-svelte';
  import { page } from '$app/stores';
  import { onMount } from 'svelte';

  import Sidebar from "@app/components/sidebar.svelte";
  import Navbar from "@app/components/navbar.svelte";
  import authService from '@app/services/auth.service';
	import { get } from 'svelte/store';



  onMount(async () => {
    const url = get(page).url;
    console.log('url :>> ', url);
    await authService.init(url);
  });

  $: title = $page.data.title;


  $: breadcrumbs = $page.url.pathname.split('/').map(pathname => ({
      home: !pathname,
      text: pathname || 'home',
      url: pathname ? $page.url.pathname.split(pathname)[0] + pathname : '/'
    })).filter(({ url }, index) => !(url === '/' && index > 0));
</script>

<div class="flex">
  <Sidebar />
  <div class="flex flex-col flex-grow">
    <Navbar />
    <div class="py-4">
      <Breadcrumb aria-label="Default breadcrumb example">
        {#each breadcrumbs as breadcrumb}
          <BreadcrumbItem home={breadcrumb.home} href={breadcrumb.url}>{breadcrumb.text}</BreadcrumbItem>
        {/each}
      </Breadcrumb>
      <div class="pt-8">
        {#if title}
          <h2 class="text-xl pb-16">{title}</h2>
        {/if}
        <slot />
      </div>
    </div>
  </div>
</div>
