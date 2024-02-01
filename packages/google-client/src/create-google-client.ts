import { createOpenApiFetchClient, withRawApi, type TFetchClient } from '@dyn/fetch-client';

import type { paths } from './gen/v1';
import { withGoogle } from './with-google';

export function createGoogleClient(
	config: TGoogleClientConfig
): TFetchClient<['base', 'openapi', 'rapi', 'google'], paths> {
	const { prefixUrl = 'https://www.googleapis.com/webfonts/v1', apiKey } = config;
	return withGoogle(
		withRawApi(
			createOpenApiFetchClient<paths>({
				prefixUrl,
				middleware: [
					async (data) => {
						const { queryParams } = data;
						const newQueryParams = queryParams ?? {};
						newQueryParams.key = apiKey;
						return { queryParams: newQueryParams };
					}
				]
			})
		)
	);
}

export interface TGoogleClientConfig {
	prefixUrl?: string;
	apiKey: string;
}
