import { HoudiniClient, type RequestHandler } from '$houdini';
import { PUBLIC_API_URL } from '$env/static/public';
import { token } from './store';
import { get } from 'svelte/store';

const requestHandler: RequestHandler = async ({
	fetch,
	text = '',
	variables = {},
	metadata,
	session
}) => {
	const url = `${PUBLIC_API_URL}/graphql`;
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
