<script lang="ts">
  import { AppRailTile } from "@skeletonlabs/skeleton";
  import Icon from '@iconify/svelte';
  import { page } from '$app/stores';

  export let href: string;
  export let icon: string;
  export let label: string;

  $: active = href === $page.url.pathname ;
</script>

<AppRailTile label={label} tag="a" href={href} class="{active ? '!bg-primary-500' : ''}">
  <Icon icon={icon} width={24} style="min-width: 24px;" />
</AppRailTile>

