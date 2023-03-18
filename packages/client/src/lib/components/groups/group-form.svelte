<script lang="ts">
	import { Input, Checkbox, Button } from '$lib/components';
	import { createForm } from 'felte';
	import { CreateGroupStore, type CreateGroup$input } from '$houdini';
	import { goto } from '$app/navigation';

	interface FormValues {
		name: string;
		private: boolean;
		slug: string;
	}

	const { form } = createForm({
		async onSubmit(dto: FormValues) {
			const input: CreateGroup$input = {
				dto
			};

			const createGroup = new CreateGroupStore();
			const createdGroup = await createGroup.mutate(input);

			const groupId = createdGroup.data?.createGroup.id;

			if (groupId) {
				goto(`/${groupId}`);
			}
		}
	});
</script>

<form use:form class="card w-96 bg-surface-50 p-4 shadow-lg">
	<header class="card-header">
		<h3 class="capitalize-first text-center text-2xl font-medium">create a group</h3>
	</header>
	<section class="space-y-4 p-4">
		<Input label="name" name="name" placeholder="my group name" required />
		<Input label="slug" name="slug" placeholder="my-group-slug" required />
		<Checkbox label="private" name="private" />
		<div class="flex justify-end">
			<Button type="submit">create</Button>
		</div>
	</section>
</form>
