import {
	hasFeatures,
	type TEnforceFeatures,
	type TFeatureKeys,
	type TFetchClient,
	type TSelectFeatures
} from '@dyn/fetch-client';

import type { paths } from './gen/v1';

export function withGoogle<GSelectedFeatureKeys extends TFeatureKeys[]>(
	fetchClient: TFetchClient<TEnforceFeatures<GSelectedFeatureKeys, ['base', 'openapi']>>
): TFetchClient<['google', ...GSelectedFeatureKeys], paths> {
	if (hasFeatures(fetchClient, ['openapi'])) {
		const googleFeature: TSelectFeatures<['google']> = {
			async getWebFonts(this: TFetchClient<['base', 'openapi'], paths>, options = {} as any) {
				return this.get('/webfonts', {
					queryParams: {
						key: 'not-set', // Set by middleware,
						...options
					}
				});
			}
		};

		// Merge existing features from the state with the new api feature
		const _fetchClient = Object.assign(fetchClient, googleFeature);

		return _fetchClient as TFetchClient<['google', ...GSelectedFeatureKeys], paths>;
	}

	throw Error('FetchClient must have "openapi" feature to use withGoogle');
}
