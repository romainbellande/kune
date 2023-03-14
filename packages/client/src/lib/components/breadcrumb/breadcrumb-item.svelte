<script lang="ts">
	export let href: string;
</script>

<div class="group flex space-x-4">
	<li class="crumb">
		<a {href}>
			<slot />
		</a>
	</li>
	<li class="crumb-separator group-last:hidden" aria-hidden>/</li>
</div>
