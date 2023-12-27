import type { TEnforceFeatures, TFeatureKeys, TFetchClient, TSelectFeatures } from '../types';

export function withApi<GPaths extends {}, GSelectedFeatureKeys extends TFeatureKeys[]>(
	fetchClient: TFetchClient<GPaths, TEnforceFeatures<GSelectedFeatureKeys, ['base']>>
): TFetchClient<GPaths, ['api', ...GSelectedFeatureKeys]> {
	const apiFeature: TSelectFeatures<GPaths, ['api']> = {
		get(this: TFetchClient<GPaths, ['base']>, path, options) {
			return this._baseFetch(path, 'GET', options);
		},
		post(this: TFetchClient<GPaths, ['base']>, path, body, options) {
			return this._baseFetch(path, 'POST', { body, ...options });
		},
		put(this: TFetchClient<GPaths, ['base']>, path, body, options) {
			return this._baseFetch(path, 'PUT', { body, ...options });
		},
		del(this: TFetchClient<GPaths, ['base']>, path, options) {
			return this._baseFetch(path, 'DELETE', options);
		}
	};

	// Merge existing features from the state with the new api feature
	const _fetchClient = Object.assign(fetchClient, apiFeature);

	return _fetchClient as TFetchClient<GPaths, ['api', ...GSelectedFeatureKeys]>;
}
