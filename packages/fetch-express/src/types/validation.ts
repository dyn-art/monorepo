import type { TRequestBody, TRequestPathParams, TRequestQueryParams } from './openapi';
import type { TParserSchema } from './parser';

// =============================================================================
// Validation Schema
// =============================================================================

// Validation schema for query parameters
export type TOpenApiQueryParamsValidationSchema<GPathOperation> =
	undefined extends TRequestQueryParams<GPathOperation> // If the queryParams can be undefined/optional
		? { querySchema: Required<TParserSchema<TRequestQueryParams<GPathOperation> | undefined>> }
		: TRequestQueryParams<GPathOperation> extends never
		? { querySchema?: Record<string, TParserSchema<any>> }
		: { querySchema: TParserSchema<TRequestQueryParams<GPathOperation>> };

// Validation schema for path parameters
export type TOpenApiPathParamsValidationSchema<GPathOperation> =
	undefined extends TRequestPathParams<GPathOperation> // If the pathParams can be undefined/optional
		? { pathSchema: TParserSchema<TRequestPathParams<GPathOperation> | undefined> }
		: TRequestPathParams<GPathOperation> extends never
		? { pathSchema?: Record<string, TParserSchema<any>> }
		: { pathSchema: TParserSchema<TRequestPathParams<GPathOperation>> };

// Validation schema for body
export type TOpenApiBodyValidationSchema<GPathOperation> =
	undefined extends TRequestBody<GPathOperation> // If the body can be undefined/optional
		? { bodySchema: TParserSchema<TRequestBody<GPathOperation> | undefined> }
		: TRequestBody<GPathOperation> extends never
		? { bodySchema?: Record<string, TParserSchema<any>> }
		: { bodySchema: TParserSchema<TRequestBody<GPathOperation>> };

export type TOpenApiValidationSchema<GPathOperation> =
	TOpenApiPathParamsValidationSchema<GPathOperation> &
		TOpenApiQueryParamsValidationSchema<GPathOperation> &
		TOpenApiBodyValidationSchema<GPathOperation>;
