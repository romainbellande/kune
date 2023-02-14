<script>
  import { page } from '$app/stores';
  import Icon from '@iconify/svelte';
  import { Sidebar, SidebarDropdownItem, SidebarDropdownWrapper, SidebarGroup, SidebarItem, SidebarWrapper } from 'flowbite-svelte';
  let spanClass = 'flex-1 ml-3 whitespace-nowrap';
  $: activeUrl = $page.url.pathname
</script>

<Sidebar>
  <SidebarWrapper>
    <SidebarGroup>
      <SidebarDropdownWrapper label="Groups">
        <svelte:fragment slot="icon">
          <Icon icon="ic:baseline-group" />
        </svelte:fragment>

        <SidebarDropdownItem label="Create" />
        <SidebarDropdownItem label="List" />
      </SidebarDropdownWrapper>
    </SidebarGroup>
  </SidebarWrapper>
</Sidebar>
