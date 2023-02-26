<script>
  import { Navbar, Button } from 'flowbite-svelte'
  import Icon from '@iconify/svelte';
</script>

<style lang="scss">
  :global(.navclass){
    --shadow-height: .5rem;
    --shadow-gradient: linear-gradient( to bottom, rgba(0, 0, 0, .1) 0%, rgba(0, 0, 0, .05) 30%, transparent 100% );

    &:after {
      content: "";
      position: absolute;
      width: 100%;
      height: var(--shadow-height);
      left: 0;
      bottom: calc(-1 * var(--shadow-height));
      background: var(--shadow-gradient);
    }
  }
</style>

<Navbar navClass="navclass relative py-3 bg-primary" color="gray">
    <Button>
      Create a group
      <Icon class="ml-2" icon="material-symbols:add-circle-outline" height={20} width={20} />
    </Button>
</Navbar>
