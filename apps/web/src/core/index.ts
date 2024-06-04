import { createApiFetchClient } from 'feature-fetch';

import { appConfnig } from '../environment';

export const appFetchClient = createApiFetchClient({
	prefixUrl: appConfnig.url.endsWith('/') ? appConfnig.url : `${appConfnig.url}/`
});
