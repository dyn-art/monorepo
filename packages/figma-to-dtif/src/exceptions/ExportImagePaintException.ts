import { extractErrorData } from '@dyn/utils';

import { NodeException } from './NodeException';

export class ExportImagePaintException extends NodeException {
	public readonly throwable?: Error;

	constructor(nodeIds: SceneNode['id'][], throwable?: unknown) {
		const errorData = throwable != null ? extractErrorData(throwable) : null;
		super(
			`Failed to export image paint${errorData != null ? ` by error: ${errorData.message}` : '!'}`,
			nodeIds
		);
		this.throwable = errorData?.error ?? undefined;
	}
}
