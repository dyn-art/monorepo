import { createApiFetchClient } from 'feature-fetch';

import { appConfig } from '../environment';

export const appFetchClient = createApiFetchClient({
	prefixUrl: appConfig.url.endsWith('/') ? appConfig.url : `${appConfig.url}/`
});
