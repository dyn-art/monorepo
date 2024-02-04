import { extractErrorData } from '@dyn/utils';

import { NodeException } from './NodeException';

export class UploadStaticDataException extends NodeException {
	public readonly throwable?: Error;

	constructor(nodeIds: SceneNode['id'][], throwable?: unknown) {
		const errorData = throwable != null ? extractErrorData(throwable) : null;
		super(
			`Failed to upload static data${errorData != null ? ` by error: ${errorData.message}` : '!'}`,
			nodeIds
		);
		this.throwable = errorData?.error ?? undefined;
	}
}
