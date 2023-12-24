import type { TFeatureKeys, TFetchClient, TSelectFeatures } from '../types';

export function withApi<GSelectedFeatureKeys extends TFeatureKeys[]>(
	fetchClient: TFetchClient<GSelectedFeatureKeys>
): TFetchClient<['api', ...GSelectedFeatureKeys]> {
	const apiFeature: TSelectFeatures<['api']> = {
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

	return _fetchClient as TFetchClient<['api', ...GSelectedFeatureKeys]>;
}
