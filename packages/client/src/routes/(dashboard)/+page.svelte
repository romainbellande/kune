<script lang="ts">
  import { isAuthenticated, auth0User, user } from '@app/store';
  import { get } from 'svelte/store';
  import './page.css';
  import type { PageData } from './$houdini';
  import { GetUserByExternalIdStore } from '$houdini';

  isAuthenticated.subscribe(async (value) => {
    if (value) {
      const getUserByExternalId = new GetUserByExternalIdStore();
      console.log("test")
      const externalId = get(auth0User)?.sub;

      if (externalId) {
        const result = await getUserByExternalId.fetch({ variables: { externalId } });
        user.set(result.data);
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
