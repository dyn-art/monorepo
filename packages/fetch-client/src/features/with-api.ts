import type { TEnforceFeatures, TFeatureKeys, TFetchClient, TSelectFeatures } from '../types';

export function withApi<GSelectedFeatureKeys extends TFeatureKeys[]>(
	fetchClient: TFetchClient<TEnforceFeatures<GSelectedFeatureKeys, ['base']>>
): TFetchClient<['api', ...GSelectedFeatureKeys]> {
	fetchClient._features.push('api');

	const apiFeature: TSelectFeatures<['api']> = {
		get(this: TFetchClient<['base']>, path, options = {}) {
			return this._baseFetch(path, 'GET', options);
		},
		post(this: TFetchClient<['base']>, path, body, options = {}) {
			return this._baseFetch(path, 'POST', { ...options, body });
		},
		put(this: TFetchClient<['base']>, path, body, options = {}) {
			return this._baseFetch(path, 'PUT', { ...options, body });
		},
		del(this: TFetchClient<['base']>, path, options = {}) {
			return this._baseFetch(path, 'DELETE', options);
		}
	};

	// Merge existing features from the state with the new api feature
	const _fetchClient = Object.assign(fetchClient, apiFeature);

	return _fetchClient as TFetchClient<['api', ...GSelectedFeatureKeys]>;
}

export function withRawApi<GSelectedFeatureKeys extends TFeatureKeys[]>(
	fetchClient: TFetchClient<TEnforceFeatures<GSelectedFeatureKeys, ['base']>>
): TFetchClient<['rapi', ...GSelectedFeatureKeys]> {
	fetchClient._features.push('rapi');

	const apiFeature: TSelectFeatures<['rapi']> = {
		rGet(this: TFetchClient<['base']>, path, options = {}) {
			return this._baseFetch(path, 'GET', options);
		},
		rPost(this: TFetchClient<['base']>, path, body, options = {}) {
			return this._baseFetch(path, 'POST', { ...options, body });
		},
		rPut(this: TFetchClient<['base']>, path, body, options = {}) {
			return this._baseFetch(path, 'PUT', { ...options, body });
		},
		rDel(this: TFetchClient<['base']>, path, options = {}) {
			return this._baseFetch(path, 'DELETE', options);
		}
	};

	// Merge existing features from the state with the new api feature
	const _fetchClient = Object.assign(fetchClient, apiFeature);

	return _fetchClient as TFetchClient<['rapi', ...GSelectedFeatureKeys]>;
}
