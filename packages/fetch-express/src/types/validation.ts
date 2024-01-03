import type { TRequestPathParams, TRequestQueryParams } from './openapi';

// TODO:

// =============================================================================
// Validation Options
// =============================================================================

// Fetch options for query parameters
export type TOpenApiQueryParamsValidationOptions<GPathOperation> =
	undefined extends TRequestQueryParams<GPathOperation> // If the queryParams can be undefined/optional
		? { queryParams?: TRequestQueryParams<GPathOperation> }
		: TRequestQueryParams<GPathOperation> extends never
		? { queryParams?: Record<string, unknown> }
		: { queryParams: TRequestQueryParams<GPathOperation> };

// Fetch options for path parameters
export type TOpenApiPathParamsValidationOptions<GPathOperation> =
	undefined extends TRequestPathParams<GPathOperation> // If the pathParams can be undefined/optional
		? { pathParams?: TRequestPathParams<GPathOperation> }
		: TRequestPathParams<GPathOperation> extends never
		? { pathParams?: Record<string, unknown> }
		: { pathParams: TRequestPathParams<GPathOperation> };
