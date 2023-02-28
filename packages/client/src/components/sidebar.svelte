<script lang="ts">
  import { page } from '$app/stores';
  import Icon from '@iconify/svelte';
  import { Sidebar, SidebarDropdownItem, SidebarDropdownWrapper, SidebarGroup, SidebarWrapper } from 'flowbite-svelte';
  $: activeUrl = $page.url.pathname;

  interface DropdownItem {
    aClass: string;
    label: string;
    href: string;
  }

  const dropdownItemAClass = 'rounded-lg flex items-center p-2 pl-11 w-full text-base font-normal text-gray-200 hover:text-gray-900 transition duration-75 group hover:bg-gray-100 dark:text-white dark:hover:bg-gray-700';

  const groupDropdownItems: DropdownItem[] = [
    {
      aClass: dropdownItemAClass,
      label: 'Create',
      href: '/groups/create'
    },
    {
      aClass: dropdownItemAClass,
      label: 'List',
      href: '/groups'
    },
  ];
</script>

<Sidebar asideClass="bg-primary w-64 text-white">
  <SidebarWrapper divClass="bg-primary">
    <div class="py-4 text-center text-xl font-bold mb-6">Kune</div>
    <div class="px-4">
      <SidebarGroup >
        <SidebarDropdownWrapper label="Groups"  btnClass="rounded-lg flex items-center p-2 w-full text-base font-normal text-gray-200 hover:text-gray-900 transition duration-75 group hover:bg-gray-100 dark:text-white dark:hover:bg-gray-700">
          <svelte:fragment slot="icon">
            <Icon icon="ic:baseline-group" />
          </svelte:fragment>
          {#each groupDropdownItems as groupItem}
            <SidebarDropdownItem aClass={groupItem.aClass} label={groupItem.label} href={groupItem.href} />
          {/each}
        </SidebarDropdownWrapper>
      </SidebarGroup>
    </div>
  </SidebarWrapper>
</Sidebar>
