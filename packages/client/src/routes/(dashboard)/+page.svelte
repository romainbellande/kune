<script>
  import { isAuthenticated } from '../../store';
  import './page.css';
</script>

{#if $isAuthenticated}
<div>home</div>
{:else}
<div>loading...</div>
{/if}
