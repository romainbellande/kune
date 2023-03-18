<script lang="ts">
	export let label: string;
	export let name: string;
	export let required = false;
</script>

<label class="flex items-center space-x-4">
	<input type="checkbox" class="checkbox border border-surface-600" {name} {required} />
	<span class="flex space-x-1 font-medium">
		<span>{label}</span>
		{#if required}
			<span class="text-red-700">*</span>
		{/if}
	</span>
</label>
