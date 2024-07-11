import { createApiFetchClient, createOpenApiFetchClient } from 'feature-fetch';
import { type paths } from '@dyn/types/core';
import { appConfig } from '@/environment';

export const coreFetchClient = createOpenApiFetchClient<paths>({
	prefixUrl: 'http://localhost:9000'
});

export const appFetchClient = createApiFetchClient({
	prefixUrl: appConfig.url.endsWith('/') ? appConfig.url : `${appConfig.url}/`
});
