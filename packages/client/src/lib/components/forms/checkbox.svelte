<script lang="ts">
  export let label: string;
  export let name: string;
  export let required = false;
</script>

<label class="flex space-x-4 items-center">
  <input type="checkbox" class="checkbox border border-surface-600" {name} {required} />
  <span class="space-x-1 font-medium flex">
		<span>{label}</span>
		{#if required}
			<span class="text-red-700">*</span>
		{/if}
	</span>
</label>
