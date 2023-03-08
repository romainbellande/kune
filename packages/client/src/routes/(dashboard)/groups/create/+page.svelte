<script lang="ts">
	import { createForm } from 'felte';
	import { Label, Input, Button } from 'flowbite-svelte';
	import { CreateGroupStore } from '$houdini';
	import type { CreateGroup$input } from '$houdini';
	import { user } from '@app/store';
	import { get } from 'svelte/store';
	import { page } from '$app/stores';

	interface FormValues {
		name: string;
	}

	const { form } = createForm({
		async onSubmit({ name }: FormValues) {
			const input: CreateGroup$input = {
				dto: {
					name
				}
			};

			const createGroup = new CreateGroupStore();
			const createdGroup = await createGroup.mutate(input);
			console.log('createdGroup :>> ', createdGroup);
		}
	});
</script>

<div>
	<form use:form class="mx-auto max-w-lg space-y-4">
		<div class="space-y-2">
			<Label for="name">Name</Label>
			<Input type="text" id="name" name="name" placeholder="my group name" required />
		</div>
		<div>
			<Button type="submit">submit</Button>
		</div>
	</form>
</div>
