import type { TEnforceFeatures, TFeatureKeys, TFetchClient, TSelectFeatures } from '../types';

export function withOpenApi<GSelectedFeatureKeys extends TFeatureKeys[]>(
	fetchClient: TFetchClient<TEnforceFeatures<GSelectedFeatureKeys, ['base']>>
): TFetchClient<['openapi', ...GSelectedFeatureKeys]> {
	const openApiFeature: TSelectFeatures<['openapi']> = {
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

	return _fetchClient as TFetchClient<['openapi', ...GSelectedFeatureKeys]>;
}
