<script lang="ts">
  import { isAuthenticated, user } from '@app/store';
  import { get } from 'svelte/store';
  import './page.css';
  import type { PageData } from './$houdini';
  import { GetUserByExternalIdStore } from '$houdini';

  isAuthenticated.subscribe(async (value) => {
    if (value) {
      const GetUserByExternalId = new GetUserByExternalIdStore();
      console.log("test")
      const externalId = get(user)?.sub;

      if (externalId) {
        const result = await GetUserByExternalId.fetch({ variables: { externalId } });

        console.log('result :>> ', result);
      }
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
