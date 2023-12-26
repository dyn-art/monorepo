import URL from 'url-parse';

import { ServiceException } from '../exceptions';

export function parseAndValidateURL(
	urlString: string,
	withSuffix = false
): {
	path: `/${string}`;
	origin: string;
} {
	try {
		const url = new URL(urlString);
		return {
			path: `${url.pathname}${withSuffix ? url.query + url.hash : ''}` as `/${string}`,
			origin: url.origin
		};
	} catch (error) {
		throw new ServiceException('#RESOLVE_URL', {
			description: `Failed to resolve url: ${urlString}`
		});
	}
}
