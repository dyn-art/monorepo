import type { COMP } from '@dyn/dtif-comp';
import { extractErrorData } from '@dyn/utils';

import { NodeException } from './NodeException';

export class ExportFontAssetException extends NodeException {
	public readonly throwable?: Error;

	constructor(fontInfo: COMP.FontInfo, nodeIds: SceneNode['id'][], throwable?: unknown) {
		const errorData = throwable != null ? extractErrorData(throwable) : null;
		super(
			`Failed to export font  '${fontInfo.family} (${
				fontInfo.variant.style
			})'${errorData != null ? ` by error: ${errorData.message}` : '!'}`,
			nodeIds
		);
		this.throwable = errorData?.error ?? undefined;
	}
}
