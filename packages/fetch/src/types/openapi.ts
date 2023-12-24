import type { TFetchResponse } from './client';

export type TOpenApiGet = <GSucessResponseBody = any, GErrorResponseBody = any>(
	path: string
) => Promise<TFetchResponse<GSucessResponseBody, GErrorResponseBody, 'json'>>;
export type TOpenApiPost = <GRequestBody, GSuccessResponseBody = any, GErrorResponseBody = any>(
	path: string,
	body: GRequestBody
) => Promise<TFetchResponse<GSuccessResponseBody, GErrorResponseBody, 'json'>>;
export type TOpenApiPut = <GRequestBody, GSuccessResponseBody = any, GErrorResponseBody = any>(
	path: string,
	body: GRequestBody
) => Promise<TFetchResponse<GSuccessResponseBody, GErrorResponseBody, 'json'>>;
export type TOpenApiDelete = <GSuccessResponseBody = any, GErrorResponseBody = any>(
	path: string
) => Promise<TFetchResponse<GSuccessResponseBody, GErrorResponseBody, 'json'>>;
