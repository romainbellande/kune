<!-- <script lang="ts">
  import { isAuthenticated, auth0User, user } from '@app/store';
  import { get } from 'svelte/store';
  import './page.css';
  import { GetUserByExternalIdStore } from '$houdini';

  isAuthenticated.subscribe(async (value) => {
    if (value) {
      const getUserByExternalId = new GetUserByExternalIdStore();
      const externalId = get(auth0User)?.sub;

      if (externalId) {
        const result = await getUserByExternalId.fetch({ variables: { externalId } });
        user.set(result.data);
      }
    }
  })

</script> -->

<div>
  Hello
</div>
