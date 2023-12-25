import type { TErrorStatus, THttpMethod, TMediaType, TOkStatus, TParseAs } from './api';

// ============================================================================
// Utility Types
// ============================================================================

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

// ============================================================================
// API Path Item Object
// ============================================================================

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

// ============================================================================
// Request Parameters
// ============================================================================

// Extracts path parameters from a generic path type
export type TRequestPathParams<GPath> = GPath extends { parameters: { path?: any } }
	? GPath['parameters']['path']
	: never;

// Filters out 'never' from path parameters for a given path type
export type TRequestPathParamsFiltered<GPath> = TRequestPathParams<GPath> extends never
	? NonNullable<TRequestPathParams<GPath>> | undefined
	: TRequestPathParams<GPath>;

// Extracts query parameters from a generic path type
export type TRequestQueryParams<GPath> = GPath extends { parameters: { query?: any } }
	? GPath['parameters']['query']
	: never;

// Filters out 'never' from query parameters for a given path type
export type TRequestQueryParamsFiltered<GPath> = TRequestQueryParams<GPath> extends never
	? NonNullable<TRequestQueryParams<GPath>> | undefined
	: TRequestQueryParams<GPath>;

// ============================================================================
// Request Body
// ============================================================================

// Extracts request body from a generic path type
export type TRequestBodyObject<GPath> = GPath extends { requestBody?: any }
	? GPath['requestBody']
	: never;

// Extracts the 'content' from the request body object
export type TRequestBodyContent<GPath> = undefined extends TRequestBodyObject<GPath>
	? TFilterKeys<NonNullable<TRequestBodyObject<GPath>>, 'content'> | undefined
	: TFilterKeys<TRequestBodyObject<GPath>, 'content'>;

// Extracts media content based on TMediaType from the request body content
export type TRequestBodyMedia<GPath> = undefined extends TRequestBodyContent<GPath>
	? TFilterKeys<NonNullable<TRequestBodyContent<GPath>>, TMediaType> | undefined
	: TFilterKeys<TRequestBodyContent<GPath>, TMediaType>;

export type TRequestBody<GPath> = TRequestBodyMedia<GPath>;

// Final type for the request body after filtering 'never' and handling optional/undefined cases
export type TRequestBodyFiltered<GPath> = TRequestBodyMedia<GPath> extends never
	? NonNullable<TRequestBodyMedia<GPath>> | undefined
	: TRequestBodyMedia<GPath>;

// ============================================================================
// Response Body
// ============================================================================

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
export type TSuccessResponseBody<GResponse> = GResponse extends { responses?: any }
	? NonNullable<TFilterKeys<TSuccessResponseContent<GResponse['responses']>, TMediaType>>
	: unknown;

// Extracts error response body for a given response type
export type TErrorResponseBody<GResponse> = GResponse extends { responses?: any }
	? NonNullable<TFilterKeys<TErrorResponseContent<GResponse['responses']>, TMediaType>>
	: unknown;

// ============================================================================
// Fetch Options
// ============================================================================

// Fetch options for query parameters
export type TFetchOptionsQueryParams<GPath> = undefined extends TRequestQueryParams<GPath>
	? { queryParams?: TRequestQueryParams<GPath> }
	: TRequestQueryParams<GPath> extends never
	? { queryParams?: Record<string, any> }
	: { queryParams: TRequestQueryParams<GPath> };

// Fetch options for path parameters
export type TFetchOptionsPathParams<GPath> = undefined extends TRequestPathParams<GPath>
	? { pathParams?: TRequestPathParams<GPath> }
	: TRequestPathParams<GPath> extends never
	? { pathParams?: Record<string, any> }
	: { pathParams: TRequestPathParams<GPath> };

// Fetch options for request body
export type TFetchOptionsBody<GPath> = undefined extends TRequestBody<GPath>
	? { body?: TRequestBody<GPath> }
	: TRequestBody<GPath> extends never
	? { body?: Record<string, any> }
	: { body: TRequestBody<GPath> };

// Base interface for fetch options
export interface TFetchOptionsBase<GPath, GParseAs extends TParseAs> {
	parseAs?: GParseAs | TParseAs; // '| TParseAs' to fix VsCode autocomplete
	headers?: Record<string, string>;
	rootFetchProps?: Omit<RequestInit, 'body' | 'headers' | 'method'>;
	middlewareProps?: Record<string, any>;
	baseUrl?: string;
}

// Combines base fetch options with query and path parameters
export type TFetchOptions<GPath, GParseAs extends TParseAs> = TFetchOptionsBase<GPath, GParseAs> &
	TFetchOptionsQueryParams<GPath> &
	TFetchOptionsPathParams<GPath>;

// Combines fetch options with request body
export type TFetchOptionsWithBody<GPath, GParseAs extends TParseAs> = TFetchOptions<
	GPath,
	GParseAs
> &
	TFetchOptionsBody<GPath>;

// ============================================================================
// API Request
// ============================================================================

export type TOpenApiGet<GPaths extends {}> = <
	GGetPaths extends TPathsWith<GPaths, 'get'> = TPathsWith<GPaths, 'get'>,
	GParseAs extends TParseAs = 'json'
>(
	path: GGetPaths | (string & Record<never, never>), // https://github.com/microsoft/TypeScript/issues/29729
	options?: TFetchOptions<TFilterKeys<GPaths[GGetPaths], 'get'>, GParseAs>
) => Promise<void>;

export type TOpenApiPost<GPaths extends {}> = <
	GPostPaths extends TPathsWith<GPaths, 'post'> = TPathsWith<GPaths, 'post'>,
	GParseAs extends TParseAs = 'json'
>(
	path: GPostPaths | (string & Record<never, never>), // https://github.com/microsoft/TypeScript/issues/29729
	body: TRequestBodyFiltered<
		'post' extends keyof GPaths[GPostPaths] ? GPaths[GPostPaths]['post'] : unknown
	>,
	options?: TFetchOptions<TFilterKeys<GPaths[GPostPaths], 'post'>, GParseAs>
) => Promise<void>;

export type TOpenApiPut<GPaths extends {}> = <
	GPutPaths extends TPathsWith<GPaths, 'put'> = TPathsWith<GPaths, 'put'>,
	GParseAs extends TParseAs = 'json'
>(
	path: GPutPaths | (string & Record<never, never>), // https://github.com/microsoft/TypeScript/issues/29729
	body: TRequestBodyFiltered<
		'put' extends keyof GPaths[GPutPaths] ? GPaths[GPutPaths]['put'] : unknown
	>,
	options?: TFetchOptions<TFilterKeys<GPaths[GPutPaths], 'put'>, GParseAs>
) => Promise<void>;

export type TOpenApiDelete<GPaths extends {}> = <
	GDeletePaths extends TPathsWith<GPaths, 'delete'> = TPathsWith<GPaths, 'delete'>,
	GParseAs extends TParseAs = 'json'
>(
	path: GDeletePaths | (string & Record<never, never>), // https://github.com/microsoft/TypeScript/issues/29729
	options?: TFetchOptions<TFilterKeys<GPaths[GDeletePaths], 'delete'>, GParseAs>
) => Promise<void>;

// ============================================================================
// OpenAPI Feature
// ============================================================================

export interface TOpenApiFeature<GPaths extends {}> {
	get: TOpenApiGet<GPaths>;
	put: TOpenApiPut<GPaths>;
	post: TOpenApiPost<GPaths>;
	del: TOpenApiDelete<GPaths>;
}
