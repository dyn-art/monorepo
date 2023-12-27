import { Err, Ok } from 'ts-results-es';

import type { TFetchClient, TFetchClientConfig, TFetchClientOptions, TURLParams } from './types';
import {
	buildURI,
	fetchWithRetries,
	mapCatchToNetworkException,
	mapResponseToRequestException,
	mergeHeaders,
	parseAndValidateURL,
	serializeQueryParams
} from './utils';

export function createFetchClient<GPaths extends {} = {}>(
	options: TFetchClientOptions = {}
): TFetchClient<GPaths, ['base']> {
	const config: TFetchClientConfig = {
		prefixUrl: options.prefixUrl ?? '',
		fetchProps: options.fetchProps ?? {},
		headers: options.headers != null ? new Headers(options.headers) : new Headers(),
		bodySerializer: options.bodySerializer ?? (null as any), // TODO:
		querySerializer: options.querySerializer ?? serializeQueryParams
	};

	// Apply default content type header
	if (!config.headers.has('Content-Type')) {
		config.headers.set('Content-Type', 'application/json; charset=utf-8');
	}

	return {
		_: null,
		_config: config,
		async _baseFetch(this: TFetchClient<GPaths, ['base']>, path, method, baseFetchOptions = {}) {
			const {
				parseAs = 'json',
				headers = {},
				bodySerializer = this._config.bodySerializer,
				querySerializer = this._config.querySerializer,
				pathParams,
				queryParams,
				body = undefined,
				pathPrefix = this._config.prefixUrl,
				fetchProps = {}
			} = baseFetchOptions;

			// Parse and validate URL to ensure that even if path is a full URL and baseUrl is an empty string,
			// the finalPath and origin can still be correctly extracted
			const { path: parsedPath, origin } = parseAndValidateURL(
				`${pathPrefix}${path}`,
				queryParams == null
			);

			const urlParams: TURLParams = {
				path: pathParams,
				query: queryParams
			};

			// Build request init object
			const mergedHeaders = mergeHeaders(headers, this._config.headers);
			const requestInit: RequestInit = {
				redirect: 'follow',
				...this._config.fetchProps,
				...fetchProps,
				method,
				headers: mergedHeaders,
				body:
					body != null
						? bodySerializer(body, mergedHeaders.get('Content-Type') ?? undefined)
						: undefined
			};

			// Build final URL
			const finalURL = buildURI(origin, {
				path: parsedPath,
				params: urlParams,
				querySerializer
			});

			// Send request
			let response: Response;
			try {
				response = await fetchWithRetries(finalURL, {
					requestInit
				});
			} catch (error) {
				return Err(mapCatchToNetworkException(error, '#ERR_NETWORK'));
			}

			// Handle ok response (parse as "parseAs" and falling back to .text() when necessary)
			if (response.ok) {
				let data: any = response.body;
				if (parseAs !== 'stream') {
					const cloned = typeof response.clone === 'function' ? response.clone() : response; // Clone method not supported by Figma sandbox environment
					try {
						data =
							typeof cloned[parseAs] === 'function' ? await cloned[parseAs]() : await cloned.text();
					} catch (error) {
						data = cloned.text();
					}
				}
				return Ok({ data, response });
			}

			// Handle errors (always parse as .json() or .text())
			return Err(await mapResponseToRequestException(response, '#ERR_UNKNOWN'));
		}
	};
}
