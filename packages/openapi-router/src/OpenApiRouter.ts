import type express from 'express';

import type {
	TBaseExpressHandler,
	TBaseValidationSchema,
	TExpressHandler,
	TFilterKeys,
	TOpenApiValidationSchema,
	TPathsWith
} from './types';

export class OpenApiRouter<GPaths extends {} = {}> {
	private readonly router: express.Router;

	constructor(router: express.Router) {
		this.router = router;
	}

	public get<
		GGetPaths extends TPathsWith<GPaths, 'get'>,
		GPathOperation extends TFilterKeys<GPaths[GGetPaths], 'get'>
	>(
		path: GGetPaths | (string & Record<never, never>),
		validation: TOpenApiValidationSchema<GPathOperation>,
		handler: TExpressHandler<GPathOperation>
	): void {
		this.router.get(
			this.formatPath(path as string),
			// eslint-disable-next-line @typescript-eslint/no-misused-promises -- Passed callback
			this.validationHandler(validation as TBaseValidationSchema),
			// eslint-disable-next-line @typescript-eslint/no-misused-promises -- Passed callback
			this.handlerWrapper(handler as TBaseExpressHandler)
		);
	}

	public post<
		GPostPaths extends TPathsWith<GPaths, 'post'>,
		GPathOperation extends TFilterKeys<GPaths[GPostPaths], 'post'>
	>(
		path: GPostPaths | (string & Record<never, never>),
		validation: TOpenApiValidationSchema<GPathOperation>,
		handler: TExpressHandler<GPathOperation>
	): void {
		this.router.post(
			this.formatPath(path as string),
			// eslint-disable-next-line @typescript-eslint/no-misused-promises -- Passed callback
			this.validationHandler(validation as TBaseValidationSchema),
			// eslint-disable-next-line @typescript-eslint/no-misused-promises -- Passed callback
			this.handlerWrapper(handler as TBaseExpressHandler)
		);
	}

	public put<
		GPutPaths extends TPathsWith<GPaths, 'put'>,
		GPathOperation extends TFilterKeys<GPaths[GPutPaths], 'put'>
	>(
		path: GPutPaths | (string & Record<never, never>),
		validation: TOpenApiValidationSchema<GPathOperation>,
		handler: TExpressHandler<GPathOperation>
	): void {
		this.router.put(
			this.formatPath(path as string),
			// eslint-disable-next-line @typescript-eslint/no-misused-promises -- Passed callback
			this.validationHandler(validation as TBaseValidationSchema),
			// eslint-disable-next-line @typescript-eslint/no-misused-promises -- Passed callback
			this.handlerWrapper(handler as TBaseExpressHandler)
		);
	}

	public del<
		GDeletePaths extends TPathsWith<GPaths, 'delete'>,
		GPathOperation extends TFilterKeys<GPaths[GDeletePaths], 'delete'>
	>(
		path: GDeletePaths | (string & Record<never, never>),
		validation: TOpenApiValidationSchema<GPathOperation>,
		handler: TExpressHandler<GPathOperation>
	): void {
		this.router.delete(
			this.formatPath(path as string),
			// eslint-disable-next-line @typescript-eslint/no-misused-promises -- Passed callback
			this.validationHandler(validation as TBaseValidationSchema),
			// eslint-disable-next-line @typescript-eslint/no-misused-promises -- Passed callback
			this.handlerWrapper(handler as TBaseExpressHandler)
		);
	}

	public formatPath(path: string): string {
		// "/users/{userId}/books/{bookId}" -> "/users/:userId/books/:bookId"
		return path.replace(/\{(?<name>\w+)\}/g, ':$<name>');
	}

	public validationHandler(validationSchema: TBaseValidationSchema): TBaseExpressHandler {
		return async (req: express.Request, res: express.Response, next: express.NextFunction) => {
			// TODO
		};
	}

	public handlerWrapper(handler: TBaseExpressHandler): TBaseExpressHandler {
		return async (req: express.Request, res: express.Response, next: express.NextFunction) => {
			// TODO
		};
	}
}
