import type { TEnforceFeatures, TFeatureKeys, TFetchClient, TSelectFeatures } from '../types';

export function withApi<GSelectedFeatureKeys extends TFeatureKeys[]>(
	fetchClient: TFetchClient<TEnforceFeatures<GSelectedFeatureKeys, ['base']>>
): TFetchClient<['api', ...GSelectedFeatureKeys]> {
	const apiFeature: TSelectFeatures<['api']> = {
		get(this: TFetchClient<['base']>, path, options) {
			return this._baseFetch(path, 'GET', options);
		},
		post(this: TFetchClient<['base']>, path, body, options) {
			return this._baseFetch(path, 'POST', { body, ...options });
		},
		put(this: TFetchClient<['base']>, path, body, options) {
			return this._baseFetch(path, 'PUT', { body, ...options });
		},
		del(this: TFetchClient<['base']>, path, options) {
			return this._baseFetch(path, 'DELETE', options);
		}
	};

	// Merge existing features from the state with the new api feature
	const _fetchClient = Object.assign(fetchClient, apiFeature);

	return _fetchClient as TFetchClient<['api', ...GSelectedFeatureKeys]>;
}
