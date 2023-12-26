import type { TErrorStatus, THttpMethod, TMediaType, TOkStatus, TParseAs } from './api';
import type { TFetchResponse } from './client';

// =============================================================================
// Utility Types
// =============================================================================

// Identifies required keys in a generic type
type TRequiredKeys<T> = {
	[GKey in keyof T]: T extends Record<GKey, T[GKey]> ? never : GKey;
}[keyof T];

// Determines if a generic type is empty
type TIsEmpty<T> = keyof T extends never ? true : false;

// Makes a generic type optional if it has no required keys
export type TOptionalIfNoRequired<T> = TIsEmpty<TRequiredKeys<T>> extends true ? T | undefined : T;

// Filters keys from a type based on a set of matchers
export type TFilterKeys<GObject, GMatchers> = {
	[GKey in keyof GObject]: GKey extends GMatchers ? GObject[GKey] : never;
}[keyof GObject];

// =============================================================================
// API Path Item Object
// =============================================================================

// Represents the base structure of API path items for a given HTTP method
export type TPathItemObject<GHttpMethod extends THttpMethod> = {
	[Method in GHttpMethod]: {
		parameters: any;
		requestBody: any;
		response: any;
	};
} & { parameters?: any };

// Retrieves paths that include a specific HTTP method
export type TPathsWith<GPaths, GHttpMethod extends THttpMethod> = {
	[GPath in keyof GPaths]: GPaths[GPath] extends { [GMethod in GHttpMethod]: any } ? GPath : never;
}[keyof GPaths];

// =============================================================================
// Request Parameters
// =============================================================================

// Extracts path parameters from a generic path type
export type TRequestPathParams<GPathOperation> = GPathOperation extends {
	parameters: { path?: any };
}
	? GPathOperation['parameters']['path']
	: never;

// Filters out 'never' from path parameters for a given path type
export type TRequestPathParamsFiltered<GPathOperation> =
	TRequestPathParams<GPathOperation> extends never
		? NonNullable<TRequestPathParams<GPathOperation>> | undefined
		: TRequestPathParams<GPathOperation>;

// Extracts query parameters from a generic path type
export type TRequestQueryParams<GPathOperation> = GPathOperation extends {
	parameters: { query?: any };
}
	? GPathOperation['parameters']['query']
	: never;

// Filters out 'never' from query parameters for a given path type
export type TRequestQueryParamsFiltered<GPathOperation> =
	TRequestQueryParams<GPathOperation> extends never
		? NonNullable<TRequestQueryParams<GPathOperation>> | undefined
		: TRequestQueryParams<GPathOperation>;

// =============================================================================
// Request Body
// =============================================================================

// Extracts request body from a generic path type
export type TRequestBodyObject<GPathOperation> = GPathOperation extends { requestBody?: any }
	? GPathOperation['requestBody']
	: never;

// Extracts the 'content' from the request body object
export type TRequestBodyContent<GPathOperation> =
	undefined extends TRequestBodyObject<GPathOperation>
		? TFilterKeys<NonNullable<TRequestBodyObject<GPathOperation>>, 'content'> | undefined
		: TFilterKeys<TRequestBodyObject<GPathOperation>, 'content'>;

// Extracts media content based on TMediaType from the request body content
export type TRequestBodyMedia<GPathOperation> =
	undefined extends TRequestBodyContent<GPathOperation>
		? TFilterKeys<NonNullable<TRequestBodyContent<GPathOperation>>, TMediaType> | undefined
		: TFilterKeys<TRequestBodyContent<GPathOperation>, TMediaType>;

// Final type for the request body after filtering 'never' and handling optional/undefined cases
export type TRequestBody<GPathOperation> = TRequestBodyMedia<GPathOperation> extends never
	? NonNullable<TRequestBodyMedia<GPathOperation>> | undefined
	: TRequestBodyMedia<GPathOperation>;

// =============================================================================
// Response Body
// =============================================================================

// Extracts successful response content for a given response type
export type TSuccessResponseContent<GResponse> = TFilterKeys<
	TFilterKeys<GResponse, TOkStatus>,
	'content'
>;

// Extracts error response content for a given response type
export type TErrorResponseContent<GResponse> = TFilterKeys<
	TFilterKeys<GResponse, TErrorStatus>,
	'content'
>;

// Extracts successful response body for a given response type
export type TSuccessResponseBody<GPathOperation> = GPathOperation extends { responses?: any }
	? NonNullable<TFilterKeys<TSuccessResponseContent<GPathOperation['responses']>, TMediaType>>
	: unknown;

// Extracts error response body for a given response type
export type TErrorResponseBody<GPathOperation> = GPathOperation extends { responses?: any }
	? NonNullable<TFilterKeys<TErrorResponseContent<GPathOperation['responses']>, TMediaType>>
	: unknown;

// =============================================================================
// Fetch Response
// =============================================================================

export type TOpenApiFetchResponse<GPathOperation, GParseAs extends TParseAs> = TFetchResponse<
	TSuccessResponseBody<GPathOperation>,
	TErrorResponseBody<GPathOperation>,
	GParseAs
>;

// =============================================================================
// Fetch Options
// =============================================================================

// Fetch options for query parameters
export type TFetchOptionsQueryParams<GPathOperation> =
	undefined extends TRequestQueryParams<GPathOperation> // If the queryParams can be undefined/optional
		? { queryParams?: TRequestQueryParams<GPathOperation> }
		: TRequestQueryParams<GPathOperation> extends never
		? { queryParams?: Record<string, unknown> }
		: { queryParams: TRequestQueryParams<GPathOperation> };

// Fetch options for path parameters
export type TFetchOptionsPathParams<GPathOperation> =
	undefined extends TRequestPathParams<GPathOperation> // If the pathParams can be undefined/optional
		? { pathParams?: TRequestPathParams<GPathOperation> }
		: TRequestPathParams<GPathOperation> extends never
		? { pathParams?: Record<string, unknown> }
		: { pathParams: TRequestPathParams<GPathOperation> };

// Base interface for fetch options
export interface TFetchOptionsBase<GPathOperation, GParseAs extends TParseAs> {
	parseAs?: GParseAs | TParseAs; // '| TParseAs' to fix VsCode autocomplete
	headers?: Record<string, string>;
	fetchProps?: Omit<RequestInit, 'body' | 'headers' | 'method'>;
	middlewareProps?: Record<string, unknown>;
	pathPrefix?: string;
}

// Combines base fetch options with query and path parameters
export type TFetchOptions<GPathOperation, GParseAs extends TParseAs> = TFetchOptionsBase<
	GPathOperation,
	GParseAs
> &
	TFetchOptionsQueryParams<GPathOperation> &
	TFetchOptionsPathParams<GPathOperation>;

// =============================================================================
// API Request
// =============================================================================

export type TOpenApiGet<GPaths extends {}> = <
	GGetPaths extends TPathsWith<GPaths, 'get'> = TPathsWith<GPaths, 'get'>,
	GPathOperation extends TFilterKeys<GPaths[GGetPaths], 'get'> = TFilterKeys<
		GPaths[GGetPaths],
		'get'
	>,
	GParseAs extends TParseAs = 'json'
>(
	path: GGetPaths | (string & Record<never, never>), // https://github.com/microsoft/TypeScript/issues/29729
	options?: TFetchOptions<GPathOperation, GParseAs>
) => Promise<TOpenApiFetchResponse<GPathOperation, GParseAs>>;

export type TOpenApiPost<GPaths extends {}> = <
	GPostPaths extends TPathsWith<GPaths, 'post'> = TPathsWith<GPaths, 'post'>,
	GPathOperation extends TFilterKeys<GPaths[GPostPaths], 'post'> = TFilterKeys<
		GPaths[GPostPaths],
		'post'
	>,
	GParseAs extends TParseAs = 'json'
>(
	path: GPostPaths | (string & Record<never, never>), // https://github.com/microsoft/TypeScript/issues/29729
	body: TRequestBody<
		'post' extends keyof GPaths[GPostPaths] ? GPaths[GPostPaths]['post'] : unknown
	>,
	options?: TFetchOptions<GPathOperation, GParseAs>
) => Promise<TOpenApiFetchResponse<GPathOperation, GParseAs>>;

export type TOpenApiPut<GPaths extends {}> = <
	GPutPaths extends TPathsWith<GPaths, 'put'> = TPathsWith<GPaths, 'put'>,
	GPathOperation = TFilterKeys<GPaths[GPutPaths], 'put'>,
	GParseAs extends TParseAs = 'json'
>(
	path: GPutPaths | (string & Record<never, never>), // https://github.com/microsoft/TypeScript/issues/29729
	body: TRequestBody<'put' extends keyof GPaths[GPutPaths] ? GPaths[GPutPaths]['put'] : unknown>,
	options?: TFetchOptions<GPathOperation, GParseAs>
) => Promise<TOpenApiFetchResponse<GPathOperation, GParseAs>>;

export type TOpenApiDelete<GPaths extends {}> = <
	GDeletePaths extends TPathsWith<GPaths, 'delete'> = TPathsWith<GPaths, 'delete'>,
	GPathOperation extends TFilterKeys<GPaths[GDeletePaths], 'delete'> = TFilterKeys<
		GPaths[GDeletePaths],
		'delete'
	>,
	GParseAs extends TParseAs = 'json'
>(
	path: GDeletePaths | (string & Record<never, never>), // https://github.com/microsoft/TypeScript/issues/29729
	options?: TFetchOptions<GPathOperation, GParseAs>
) => Promise<TOpenApiFetchResponse<GPathOperation, GParseAs>>;

// =============================================================================
// OpenAPI Feature
// =============================================================================

export interface TOpenApiFeature<GPaths extends {}> {
	get: TOpenApiGet<GPaths>;
	put: TOpenApiPut<GPaths>;
	post: TOpenApiPost<GPaths>;
	del: TOpenApiDelete<GPaths>;
}
