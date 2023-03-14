<script lang="ts">
	export let label: string;
	export let required = false;
	export let placeholder = '';
	export let name: string;
	export let type = 'text';
</script>

<label class="flex flex-col space-y-2">
	<span class="space-x-1 font-medium flex">
		<span class="capitalize-first ">{label}</span>
		{#if required}
			<span class="text-red-700">*</span>
		{/if}
	</span>
	<input class="input bg-surface-500" {type} {name} {placeholder} {required} />
</label>
