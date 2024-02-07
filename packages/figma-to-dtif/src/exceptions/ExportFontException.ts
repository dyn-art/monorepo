import type { COMP } from '@dyn/dtif';
import { extractErrorData } from '@dyn/utils';

import { NodeException } from './NodeException';

export class ExportFontException extends NodeException {
	public readonly throwable?: Error;

	constructor(fontMetadata: COMP.FontMetadata, nodeIds: SceneNode['id'][], throwable?: unknown) {
		const errorData = throwable != null ? extractErrorData(throwable) : null;
		super(
			`Failed to export font  '${fontMetadata.family}  (${fontMetadata.style})'${
				errorData != null ? ` by error: ${errorData.message}` : '!'
			}`,
			nodeIds
		);
		this.throwable = errorData?.error ?? undefined;
	}
}
