import { createApiFetchClient } from '@dyn/fetch-client';

import { appConfnig } from '../environment';

export const appFetchClient = createApiFetchClient({
	prefixUrl: appConfnig.url.endsWith('/') ? appConfnig.url : `${appConfnig.url}/`
});
