import type { TEnforceFeatures, TFeatureKeys, TFetchClient, TSelectFeatures } from '../types';

export function withOpenApi<
	GPaths extends {},
	GSelectedFeatureKeys extends TFeatureKeys[] = ['base']
>(
	fetchClient: TFetchClient<GPaths, TEnforceFeatures<GSelectedFeatureKeys, ['base']>>
): TFetchClient<GPaths, ['openapi', ...GSelectedFeatureKeys]> {
	const openApiFeature: TSelectFeatures<GPaths, ['openapi']> = {
		get(this: TFetchClient<GPaths, ['base']>, path, options) {
			return this._baseFetch(path as string, 'GET', {
				parseAs: options?.parseAs,
				headers: options?.headers,
				fetchProps: options?.fetchProps,
				// TODO: middlewareProps
				pathPrefix: options?.pathPrefix,
				querySerializer: options?.querySerializer as any,
				bodySerializer: options?.bodySerializer,
				pathParams: options?.pathParams,
				queryParams: options?.queryParams
			});
		},
		post(this: TFetchClient<GPaths, ['base']>, path, body, options) {
			return this._baseFetch(path as string, 'POST', {
				body,
				parseAs: options?.parseAs,
				headers: options?.headers,
				fetchProps: options?.fetchProps,
				// TODO: middlewareProps
				pathPrefix: options?.pathPrefix,
				querySerializer: options?.querySerializer as any,
				bodySerializer: options?.bodySerializer,
				pathParams: options?.pathParams,
				queryParams: options?.queryParams
			});
		},
		put(this: TFetchClient<GPaths, ['base']>, path, body, options) {
			return this._baseFetch(path as string, 'PUT', {
				body,
				parseAs: options?.parseAs,
				headers: options?.headers,
				fetchProps: options?.fetchProps,
				// TODO: middlewareProps
				pathPrefix: options?.pathPrefix,
				querySerializer: options?.querySerializer as any,
				bodySerializer: options?.bodySerializer,
				pathParams: options?.pathParams,
				queryParams: options?.queryParams
			});
		},
		del(this: TFetchClient<GPaths, ['base']>, path, options) {
			return this._baseFetch(path as string, 'DELETE', {
				parseAs: options?.parseAs,
				headers: options?.headers,
				fetchProps: options?.fetchProps,
				// TODO: middlewareProps
				pathPrefix: options?.pathPrefix,
				querySerializer: options?.querySerializer as any,
				bodySerializer: options?.bodySerializer,
				pathParams: options?.pathParams,
				queryParams: options?.queryParams
			});
		}
	};

	// Merge existing features from the state with the new openapi feature
	const _fetchClient = Object.assign(fetchClient, openApiFeature);

	return _fetchClient as TFetchClient<GPaths, ['openapi', ...GSelectedFeatureKeys]>;
}
