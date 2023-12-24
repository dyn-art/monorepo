import type { TEnforceFeatures, TFeatureKeys, TFetchClient, TSelectFeatures } from '../types';

export function withOpenApi<
	GPaths extends {},
	GSelectedFeatureKeys extends TFeatureKeys[] = ['base']
>(
	fetchClient: TFetchClient<GPaths, TEnforceFeatures<GSelectedFeatureKeys, ['base']>>
): TFetchClient<GPaths, ['openapi', ...GSelectedFeatureKeys]> {
	const openApiFeature: TSelectFeatures<GPaths, ['openapi']> = {
		get() {
			return null as any;
		},
		post() {
			return null as any;
		},
		put() {
			return null as any;
		},
		del() {
			return null as any;
		}
	};

	// Merge existing features from the state with the new openapi feature
	const _fetchClient = Object.assign(fetchClient, openApiFeature);

	return _fetchClient as TFetchClient<GPaths, ['openapi', ...GSelectedFeatureKeys]>;
}
