<script lang="ts">
  import Sidebar from "@app/components/sidebar.svelte";
  import Navbar from "@app/components/navbar.svelte";
</script>

<div class="flex">
  <Sidebar />
  <div class="flex flex-col flex-grow">
    <Navbar />
    <slot />
  </div>
</div>
