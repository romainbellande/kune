<script lang="ts">
	export let color = 'bg-primary-500';
	export let type: 'button' | 'submit' | 'reset' | null | undefined = 'button';
</script>

<button {type} class="btn variant-filled {color}">
	<span class="capitalize-first">
		<slot />
	</span>
</button>
