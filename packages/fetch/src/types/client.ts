import type { Result } from 'ts-results-es';

import type { NetworkException, RequestException, ServiceException } from '../exceptions';
import type { TParseAs, TRequestMethod } from './api';
import type { TFeatureKeys, TSelectFeatures } from './features';

export type TFetchClient<GSelectedFeatureKeys extends TFeatureKeys[], GPaths extends {} = {}> = {
	_config: TFetchClientConfig;
	_baseFetch: <
		GSuccessResponseBody = any,
		GErrorResponseBody = any,
		GParseAs extends TParseAs = 'json'
	>(
		path: string,
		method: TRequestMethod,
		options: TFetchOptionsWithBody<GParseAs>
	) => Promise<TFetchResponse<GSuccessResponseBody, GErrorResponseBody, GParseAs>>;
} & TSelectFeatures<GSelectedFeatureKeys, GPaths>;

// =============================================================================
// Fetch Client Options & Config
// =============================================================================

export interface TFetchClientConfig {
	prefixUrl: string;
	querySerializer: TQuerySerializer;
	bodySerializer: TBodySerializer;
	headers: Headers;
	fetchProps: Omit<RequestInit, 'body' | 'method' | 'headers'>;
}

export type TFetchClientOptions = Partial<Omit<TFetchClientConfig, 'headers'>> & {
	headers?: RequestInit['headers'];
};

// ============================================================================
// Serializer Methods
// ============================================================================

export type TQuerySerializer<
	GQueryParams extends Record<string, unknown> = Record<string, unknown>
> = (query: GQueryParams) => string;

export type TBodySerializer<
	GBody = any,
	GResult extends RequestInit['body'] = RequestInit['body']
> = (body: GBody, contentType?: string) => GResult;

// =============================================================================
// Fetch Options
// =============================================================================

export type TFetchOptionsWithBody<GParseAs extends TParseAs> = {
	body?: RequestInit['body']; // TODO: Only if POST or PUT
} & TFetchOptions<GParseAs>;

export type TFetchOptions<GParseAs extends TParseAs> = {
	queryParams?: TURLParams['query'];
	pathParams?: TURLParams['path'];
	querySerializer?: TQuerySerializer;
	bodySerializer?: TBodySerializer;
} & TBaseFetchOptions<GParseAs>;

export interface TBaseFetchOptions<GParseAs extends TParseAs> {
	parseAs?: GParseAs | TParseAs; // '| TParseAs' to fix VsCode autocomplete
	headers?: RequestInit['headers'];
	pathPrefix?: string;
	fetchProps?: Omit<RequestInit, 'body' | 'method'>;
	middlewareProps?: Record<string, unknown>;
}

export interface TURLParams {
	query?: Record<string, unknown>;
	path?: Record<string, unknown>;
}

// =============================================================================
// Fetch Response
// =============================================================================

export type TResponseBodyWithParseAs<
	GResponseBody,
	GParseAs extends TParseAs
> = GParseAs extends 'json'
	? GResponseBody
	: GParseAs extends 'text'
	? Awaited<ReturnType<Response['text']>>
	: GParseAs extends 'blob'
	? Awaited<ReturnType<Response['blob']>>
	: GParseAs extends 'arrayBuffer'
	? Awaited<ReturnType<Response['arrayBuffer']>>
	: GParseAs extends 'stream'
	? Response['body']
	: never;

export interface TFetchResponseSuccess<GSuccessResponseBody, GParseAs extends TParseAs> {
	data: TResponseBodyWithParseAs<GSuccessResponseBody, GParseAs>;
	response: Response;
}

export type TFetchResponseError<GErrorResponseBody> =
	| NetworkException
	| RequestException<GErrorResponseBody>
	| ServiceException;

export type TFetchResponse<
	GSuccessResponseBody,
	GErrorResponseBody,
	GParseAs extends TParseAs
> = Result<
	TFetchResponseSuccess<GSuccessResponseBody, GParseAs>,
	TFetchResponseError<GErrorResponseBody>
>;
