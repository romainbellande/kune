import { HoudiniClient, type RequestHandler } from '$houdini';
import { PUBLIC_API_URL } from '$env/static/public';

const requestHandler: RequestHandler = async ({
	fetch,
	text = '',
	variables = {},
	session
}) => {
	console.log('session :>> ', session);
	const { accessToken } = session;
	const url = `${PUBLIC_API_URL}/graphql`;
	const result = await fetch(url, {
		method: 'POST',
		headers: {
			'Content-Type': 'application/json',
			Authorization: `Bearer ${accessToken}`
		},
		body: JSON.stringify({
			query: text,
			variables
		})
	});
	return await result.json();
}

export default new HoudiniClient(requestHandler);
