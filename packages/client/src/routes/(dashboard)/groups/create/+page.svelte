<script lang="ts">
	import { createForm } from 'felte';
	import { Label, Input, Button, Card, Checkbox } from 'flowbite-svelte';
	import { CreateGroupStore } from '$houdini';
	import type { CreateGroup$input } from '$houdini';

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

<form use:form class="mx-auto mt-8">
	<Card padding="xl" size="xl">
		<div class="space-y-10 w-96">
			<h2 class="capitalize-first text-slate-700 text-2xl font-medium">create a group</h2>
			<div class="space-y-2">
				<Label for="name" class="text-slate-700 capitalize-first font-medium">name</Label>
				<Input  type="text" id="name" name="name" placeholder="my group name" required />
			</div>
			<Checkbox>
				<span class="ml-4 font-medium">private</span>
			</Checkbox>
			<div class="flex justify-end">
				<Button type="submit">
					<span class="capitalize-first">create</span>
				</Button>
			</div>
		</div>
	</Card>
</form>
