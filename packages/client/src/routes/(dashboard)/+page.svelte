<script lang="ts">
  import { isAuthenticated } from '@app/store';
  import './page.css';
  import type { PageData } from './$houdini';
  import { GetUserByExternalIdStore } from '$houdini';


  isAuthenticated.subscribe(async (value) => {
    if (value) {
      const GetUserByExternalId = new GetUserByExternalIdStore();
      console.log("test")
      const result = await GetUserByExternalId.fetch({ variables: { externalId: 'foo' } });

      console.log('result :>> ', result);
    }
  })

</script>

{#if $isAuthenticated}
<div>
  Hello
</div>
{:else}
<div>loading...</div>
{/if}
