import type { TParseAs } from './api';
import type { TFetchResponse } from './client';

export type TOpenApiGet = <
	GSucessResponseBody = any,
	GErrorResponseBody = any,
	GParseAs extends TParseAs = 'json'
>(
	path: string
) => Promise<TFetchResponse<GSucessResponseBody, GErrorResponseBody, GParseAs>>;
export type TOpenApiPost = <
	GSuccessResponseBody = any,
	GErrorResponseBody = any,
	GRequestBody = any,
	GParseAs extends TParseAs = 'json'
>(
	path: string,
	body: GRequestBody
) => Promise<TFetchResponse<GSuccessResponseBody, GErrorResponseBody, GParseAs>>;
export type TOpenApiPut = <
	GSuccessResponseBody = any,
	GErrorResponseBody = any,
	GRequestBody = any,
	GParseAs extends TParseAs = 'json'
>(
	path: string,
	body: GRequestBody
) => Promise<TFetchResponse<GSuccessResponseBody, GErrorResponseBody, GParseAs>>;
export type TOpenApiDelete = <
	GSuccessResponseBody = any,
	GErrorResponseBody = any,
	GParseAs extends TParseAs = 'json'
>(
	path: string
) => Promise<TFetchResponse<GSuccessResponseBody, GErrorResponseBody, GParseAs>>;

export interface TOpenApiFeature {
	get: TOpenApiGet;
	put: TOpenApiPut;
	post: TOpenApiPost;
	del: TOpenApiDelete;
}
