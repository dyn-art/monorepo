import type { TExpressRoute, TFilterKeys, TOpenApiValidationSchema, TPathsWith } from './types';

export class FetchExpress<GPaths extends {} = {}> {
	public get<
		GGetPaths extends TPathsWith<GPaths, 'get'>,
		GPathOperation extends TFilterKeys<GPaths[GGetPaths], 'get'>
	>(
		path: GGetPaths | (string & Record<never, never>),
		validation: TOpenApiValidationSchema<GPathOperation>,
		handler: TExpressRoute<GPathOperation>
	): void {
		//TODO:
	}

	public post<
		GPostPaths extends TPathsWith<GPaths, 'post'>,
		GPathOperation extends TFilterKeys<GPaths[GPostPaths], 'post'>
	>(
		path: GPostPaths | (string & Record<never, never>),
		validation: TOpenApiValidationSchema<GPathOperation>,
		handler: TExpressRoute<GPathOperation>
	): void {
		//TODO:
	}

	public put<
		GPutPaths extends TPathsWith<GPaths, 'put'>,
		GPathOperation extends TFilterKeys<GPaths[GPutPaths], 'put'>
	>(
		path: GPutPaths | (string & Record<never, never>),
		validation: TOpenApiValidationSchema<GPathOperation>,
		handler: TExpressRoute<GPathOperation>
	): void {
		//TODO:
	}

	public del<
		GDeletePaths extends TPathsWith<GPaths, 'delete'>,
		GPathOperation extends TFilterKeys<GPaths[GDeletePaths], 'delete'>
	>(
		path: GDeletePaths | (string & Record<never, never>),
		validation: TOpenApiValidationSchema<GPathOperation>,
		handler: TExpressRoute<GPathOperation>
	): void {
		//TODO:
	}
}
