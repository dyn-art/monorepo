import type express from 'express';
import type * as core from 'express-serve-static-core';

import type {
	TFilterKeys,
	TPathsWith,
	TRequestBody,
	TRequestPathParams,
	TRequestQueryParams,
	TSuccessResponseBody
} from './openapi';

// =============================================================================
// Request Options
// =============================================================================

// Request options for query parameters
export type TOpenApiQueryParamsRequestOptions<GPathOperation> =
	undefined extends TRequestQueryParams<GPathOperation> // If the queryParams can be undefined/optional
		? TRequestQueryParams<GPathOperation>
		: TRequestQueryParams<GPathOperation> extends never
		? core.Query
		: TRequestQueryParams<GPathOperation>;

// Request options for path parameters
export type TOpenApiPathParamsRequestOptions<GPathOperation> =
	undefined extends TRequestPathParams<GPathOperation> // If the pathParams can be undefined/optional
		? TRequestPathParams<GPathOperation>
		: TRequestPathParams<GPathOperation> extends never
		? core.ParamsDictionary
		: TRequestPathParams<GPathOperation>;

// =============================================================================
// Request
// =============================================================================

export type TOpenApiExpressRequest<GPathOperation> = express.Request<
	TOpenApiPathParamsRequestOptions<GPathOperation>, // Params
	TSuccessResponseBody<GPathOperation>, // ResBody
	TRequestBody<GPathOperation>, // ReqBody
	TOpenApiQueryParamsRequestOptions<GPathOperation> // ReqQuery
>;

// =============================================================================
// Response
// =============================================================================

export type TOpenApiExpressResponse<GPathOperation> = express.Response<
	TSuccessResponseBody<GPathOperation> // ResBody
>;

// =============================================================================
// Route
// =============================================================================

export type TExpressGetRoute<GPaths extends {}> = <
	GGetPaths extends TPathsWith<GPaths, 'get'>,
	GPathOperation extends TFilterKeys<GPaths[GGetPaths], 'get'>
>(
	req: TOpenApiExpressRequest<GPathOperation>,
	res: TOpenApiExpressResponse<GPathOperation>,
	next: express.NextFunction
) => Promise<void>;

export type TExpressPostRoute<GPaths extends {}> = <
	GPostPaths extends TPathsWith<GPaths, 'post'>,
	GPathOperation extends TFilterKeys<GPaths[GPostPaths], 'post'>
>(
	req: TOpenApiExpressRequest<GPathOperation>,
	res: TOpenApiExpressResponse<GPathOperation>,
	next: express.NextFunction
) => Promise<void>;

export type TExpressPutRoute<GPaths extends {}> = <
	GPutPaths extends TPathsWith<GPaths, 'put'>,
	GPathOperation extends TFilterKeys<GPaths[GPutPaths], 'put'>
>(
	req: TOpenApiExpressRequest<GPathOperation>,
	res: TOpenApiExpressResponse<GPathOperation>,
	next: express.NextFunction
) => Promise<void>;

export type TExpressDeleteRoute<GPaths extends {}> = <
	GDeletePaths extends TPathsWith<GPaths, 'delete'>,
	GPathOperation extends TFilterKeys<GPaths[GDeletePaths], 'delete'>
>(
	req: TOpenApiExpressRequest<GPathOperation>,
	res: TOpenApiExpressResponse<GPathOperation>,
	next: express.NextFunction
) => Promise<void>;
