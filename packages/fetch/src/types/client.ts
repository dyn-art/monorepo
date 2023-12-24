import type { ReadableStream } from 'node:stream/web';
import type { Result } from 'ts-results-es';

import type { NetworkException, RequestException, ServiceException } from '../exceptions';
import type { TParseAs, TRequestMethod } from './api';
import type { TFeatureKeys, TSelectFeatures } from './features';

export type TFetchClient<GSelectedFeatureKeys extends TFeatureKeys[] = []> = {
	_config: TFetchClientConfig;
	_baseFetch: <GSuccessResponseBody = any, GErrorResponseBody = any>(
		path: string,
		method: TRequestMethod,
		options: TBaseFetchOptions
	) => Promise<TFetchResponse<GSuccessResponseBody, GErrorResponseBody, 'json'>>;
} & TSelectFeatures<GSelectedFeatureKeys>;

interface TFetchClientConfig {
	prefixUrl: string;
}

interface TBaseFetchOptions {
	parseAs?: TParseAs;
	headers?: Record<string, string>;
	prefixUrl?: string;
}

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

export interface TFetchResponseSuccess<GSuccessResponseBody, GParseAs extends TParseAs> {
	isError: false;
	data: TResponseBodyWithParseAs<GSuccessResponseBody, GParseAs>;
	raw: Response;
}

export interface TFetchResponseError<GErrorResponseBody> {
	isError: true;
	error: NetworkException | RequestException<GErrorResponseBody> | ServiceException;
	raw: Response | null;
}

export type TFetchResponse<
	GSuccessResponseBody,
	GErrorResponseBody,
	GParseAs extends TParseAs
> = Result<
	TFetchResponseSuccess<GSuccessResponseBody, GParseAs>,
	TFetchResponseError<GErrorResponseBody>
>;
