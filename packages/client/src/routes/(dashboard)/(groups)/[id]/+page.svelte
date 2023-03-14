<script lang="ts">
	import { Page } from '$lib/components';
	import Icon from '@iconify/svelte';
	import Users from './users.svelte';
</script>

<Page>
	<svelte:fragment slot="header">
		<h3 class="flex items-center space-x-4 text-2xl font-medium">
			<span>My Group</span>
			<button type="button" class="bg-initial btn-icon">
				<Icon
					icon="material-symbols:exit-to-app"
					class="text-error-500"
					width={24}
					style="min-width: 24px;"
				/>
			</button>
		</h3>
	</svelte:fragment>
	<div class="flex flex-col space-y-4">
		<p>
			Lorem ipsum dolor sit amet, consectetur adipiscing elit. Integer quis eros a quam convallis
			ultrices. Integer eu scelerisque nisi. Nulla rutrum lectus in sem vehicula luctus. Nullam
			lorem sapien, dignissim accumsan facilisis ut, porta vel est. Nullam elementum interdum
			venenatis. Phasellus vulputate, nisl et congue pulvinar, velit velit tempus leo, eleifend
			finibus purus lorem ultrices turpis. Nam commodo eleifend mi quis commodo.
		</p>
		<Users />
	</div>
</Page>
