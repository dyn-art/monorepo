import { extractErrorData } from '@dyn/utils';

import { NodeException } from './NodeException';

export class ExportNodeException extends NodeException {
	public readonly throwable?: Error;

	constructor(format: string, node: SceneNode, throwable?: unknown) {
		const errorData = throwable != null ? extractErrorData(throwable) : null;
		super(
			`Failed to export node '${node.name}' as ${format}${
				errorData != null ? ` by error: ${errorData.message}` : '!'
			}`,
			node.id
		);
		this.throwable = errorData?.error ?? undefined;
	}
}
