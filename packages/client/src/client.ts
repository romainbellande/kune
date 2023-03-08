import { HoudiniClient } from '$houdini';
import { PUBLIC_API_URL } from '$env/static/public';

export default new HoudiniClient({
	url: `${PUBLIC_API_URL}/graphql`,
	fetchParams({ session }) {
		console.log('session :>> ', session);
		const accessToken = session?.accessToken;

		return {
			headers: {
				'Content-Type': 'application/json',
				Authorization: `Bearer ${accessToken}`
			}
		};
	}
});
