import type { TEnforceFeatures, TFeatureKeys, TFetchClient, TSelectFeatures } from '../types';

export function withApi<GPaths extends {}, GSelectedFeatureKeys extends TFeatureKeys[]>(
	fetchClient: TFetchClient<GPaths, TEnforceFeatures<GSelectedFeatureKeys, ['base']>>
): TFetchClient<GPaths, ['api', ...GSelectedFeatureKeys]> {
	const apiFeature: TSelectFeatures<GPaths, ['api']> = {
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

	// Merge existing features from the state with the new api feature
	const _fetchClient = Object.assign(fetchClient, apiFeature);

	return _fetchClient as TFetchClient<GPaths, ['api', ...GSelectedFeatureKeys]>;
}
