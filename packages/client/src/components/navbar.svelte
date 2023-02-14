<script>
  import { Navbar, NavBrand, NavLi, NavUl, NavHamburger, Button } from 'flowbite-svelte'
  import Icon from '@iconify/svelte';
</script>

<Navbar>
    <Button outline>
      Create a group
      <Icon class="ml-2" icon="material-symbols:add-circle-outline" height={20} width={20} />
    </Button>
</Navbar>
