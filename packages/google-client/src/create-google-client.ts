import { createOpenApiFetchClient, type TFetchClient } from '@dyn/fetch-client';

import { googleConfig } from './environment';
import type { paths } from './gen/v1';
import { withGoogle } from './with-google';

export function createGoogleClient(
	options: TGoogleClientOptions = {}
): TFetchClient<['base', 'openapi', 'google'], paths> {
	const { prefixUrl = googleConfig.baseUrl, apiKey = googleConfig.auth.apiKey } = options;
	return withGoogle(
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
	);
}

export interface TGoogleClientOptions {
	prefixUrl?: string;
	apiKey?: string;
}
