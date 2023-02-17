<script lang="ts">
  import { page } from '$app/stores';
  import Icon from '@iconify/svelte';
  import { Sidebar, SidebarDropdownItem, SidebarDropdownWrapper, SidebarGroup, SidebarWrapper } from 'flowbite-svelte';
  $: activeUrl = $page.url.pathname;
</script>

<Sidebar>
  <SidebarWrapper>
    <div class="py-2 text-center text-xl font-bold">Kune</div>
    <SidebarGroup>
      <SidebarDropdownWrapper label="Groups">
        <svelte:fragment slot="icon">
          <Icon icon="ic:baseline-group" />
        </svelte:fragment>

        <SidebarDropdownItem label="Create" href="/groups/create" />
        <SidebarDropdownItem label="List" href="/groups" />
        <SidebarDropdownItem label="Search" href="/groups/search" />
      </SidebarDropdownWrapper>
    </SidebarGroup>
  </SidebarWrapper>
</Sidebar>
