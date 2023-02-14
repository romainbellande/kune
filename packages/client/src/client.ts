import { HoudiniClient, type RequestHandler } from '$houdini';
import { token } from './store';
import { get } from 'svelte/store';

const requestHandler: RequestHandler = async ({
	fetch,
	text = '',
	variables = {},
	metadata,
	session
}) => {
	const url = 'http://127.0.0.1:3000/graphql';
	const result = await fetch(url, {
		method: 'POST',
		headers: {
			'Content-Type': 'application/json',
			Authorization: `Bearer ${get(token)}`
		},
		body: JSON.stringify({
			query: text,
			variables
		})
	});
	return await result.json();
}

export default new HoudiniClient(requestHandler);
