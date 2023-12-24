import type { ReadableStream } from 'node:stream/web';

import type { TErrorStatus, THttpMethod, TMediaType, TOkStatus, TParseAs } from './api';

// ============================================================================
// Utils
// ============================================================================

type TRequiredKeys<T> = {
	[K in keyof T]: T extends Record<K, T[K]> ? never : K;
} extends { [_ in keyof T]: infer U }
	? U
	: never;

type TIsEmpty<T> = keyof T extends never ? true : false;

export type TOptionalIfNoRequired<T> = TIsEmpty<TRequiredKeys<T>> extends true ? T | undefined : T;

// Find first match of multiple keys
export type TFilterKeys<Obj, Matchers> = {
	[K in keyof Obj]: K extends Matchers ? Obj[K] : never;
}[keyof Obj];

// Base path item object structure
export type TPathItemObject = {
	[GHttpMethod in THttpMethod]: {
		parameters: any;
		requestBody: any;
		response: any;
	};
} & { parameters?: any };

// Get a union of paths which have http method
export type TPathsWith<
	GPaths extends Record<string, TPathItemObject>,
	GPathnameMethod extends THttpMethod
> = {
	[GPathname in keyof GPaths]: GPaths[GPathname] extends {
		[K in GPathnameMethod]: any;
	}
		? GPathname
		: never;
}[keyof GPaths];

// ============================================================================
// Request parameters
// ============================================================================

export type TRequestPathParamsObject<T> = T extends {
	parameters: { path?: any };
}
	? T['parameters']['path']
	: never;

export type TRequestPathParams<T> = TRequestPathParamsObject<T>;

export type TRequestPathParamsFilteredNever<T> = TRequestPathParamsObject<T> extends never
	? NonNullable<TRequestPathParamsObject<T>> | undefined
	: TRequestPathParamsObject<T>;

export type TRequestQueryParamsObject<T> = T extends {
	parameters: { query?: any };
}
	? T['parameters']['query']
	: never;

export type TRequestQueryParams<T> = TRequestQueryParamsObject<T>;

export type TRequestQueryParamsFilteredNever<T> = TRequestQueryParamsObject<T> extends never
	? NonNullable<TRequestQueryParamsObject<T>> | undefined
	: TRequestQueryParamsObject<T>;

// ============================================================================
// Request body
// ============================================================================

export type TRequestBodyObject<T> = T extends { requestBody?: any } ? T['requestBody'] : never;

export type TRequestBodyContent<T> = undefined extends TRequestBodyObject<T>
	? TFilterKeys<NonNullable<TRequestBodyObject<T>>, 'content'> | undefined
	: TFilterKeys<TRequestBodyObject<T>, 'content'>;

export type TRequestBodyMedia<T> = undefined extends TRequestBodyContent<T>
	? TFilterKeys<NonNullable<TRequestBodyContent<T>>, TMediaType> | undefined
	: TFilterKeys<TRequestBodyContent<T>, TMediaType>;

export type TRequestBody<T> = TRequestBodyMedia<T>;

export type TRequestBodyFilteredNever<T> = TRequestBodyMedia<T> extends never
	? NonNullable<TRequestBodyMedia<T>> | undefined
	: TRequestBodyMedia<T>;

// ============================================================================
// Response body
// ============================================================================

export type TSuccessResponseContent<T> = TFilterKeys<TFilterKeys<T, TOkStatus>, 'content'>;

export type TErrorResponseContent<T> = TFilterKeys<TFilterKeys<T, TErrorStatus>, 'content'>;

export type TSuccessResponseBody<T> = T extends { responses?: any }
	? NonNullable<TFilterKeys<TSuccessResponseContent<T['responses']>, TMediaType>>
	: unknown;

export type TErrorResponseBody<T> = T extends { responses?: any }
	? NonNullable<TFilterKeys<TErrorResponseContent<T['responses']>, TMediaType>>
	: unknown;

export type TResponseBody<T> = TSuccessResponseBody<T>; // No ErrorResponse as errors are handled via Exceptions

export type TResponseBodyWithParseAs<
	GResponseBody,
	GParseAs extends TParseAs = 'json'
> = GParseAs extends 'json'
	? GResponseBody
	: GParseAs extends 'text'
	? string
	: GParseAs extends 'blob'
	? Blob
	: GParseAs extends 'arrayBuffer'
	? ArrayBuffer
	: GParseAs extends 'stream'
	? ReadableStream
	: never;

// ============================================================================
// Fetch options
// ============================================================================

export type TFetchOptionsQueryParamsPart<T> = undefined extends TRequestQueryParams<T> // Note: 'undefined extends xyz' behaves different than 'xyz extends undefined'
	? {
			queryParams?: TRequestQueryParams<T>;
	  }
	: TRequestQueryParams<T> extends never
	? { queryParams?: Record<string, any> }
	: { queryParams: TRequestQueryParams<T> };

export type TFetchOptionsPathParamsPart<T> = undefined extends TRequestPathParams<T>
	? { pathParams?: TRequestPathParams<T> }
	: TRequestPathParams<T> extends never
	? { pathParams?: Record<string, any> }
	: { pathParams: TRequestPathParams<T> };

export type TFetchOptionsBodyPart<T> = undefined extends TRequestBody<T>
	? { body?: TRequestBody<T> }
	: TRequestBody<T> extends never
	? { body?: any }
	: { body: TRequestBody<T> };

export interface TFetchOptionsBase<T, GParseAs extends TParseAs> {
	parseAs?: GParseAs | TParseAs; // '| TParseAs' to fix VsCode autocomplete
	headers?: Record<string, string>;
	rootFetchProps?: Omit<RequestInit, 'body' | 'headers' | 'method'>;
	middlewareProps?: Record<string, any>;
	baseUrl?: string;
}

export type TFetchOptions<T, GParseAs extends TParseAs> = TFetchOptionsBase<T, GParseAs> &
	TFetchOptionsQueryParamsPart<T> &
	TFetchOptionsPathParamsPart<T>;
export type TFetchOptionsWithBody<T, GParseAs extends TParseAs> = TFetchOptions<T, GParseAs> &
	TFetchOptionsBodyPart<T>;

// ============================================================================
// Requests
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
	path: string,
	body: TRequestBodyFilteredNever<
		'post' extends keyof GPaths[GPostPaths] ? GPaths[GPostPaths]['post'] : unknown
	>,
	options?: TFetchOptions<TFilterKeys<GPaths[GPostPaths], 'post'>, GParseAs>
) => Promise<void>;

export type TOpenApiPut<GPaths extends {}> = <
	GPutPaths extends TPathsWith<GPaths, 'put'> = TPathsWith<GPaths, 'put'>,
	GParseAs extends TParseAs = 'json'
>(
	path: GPutPaths | (string & Record<never, never>), // https://github.com/microsoft/TypeScript/issues/29729
	body: TRequestBodyFilteredNever<
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

export interface TOpenApiFeature<GPaths extends {}> {
	get: TOpenApiGet<GPaths>;
	put: TOpenApiPut<GPaths>;
	post: TOpenApiPost<GPaths>;
	del: TOpenApiDelete<GPaths>;
}
