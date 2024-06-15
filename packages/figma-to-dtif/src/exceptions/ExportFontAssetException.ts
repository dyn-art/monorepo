import { extractErrorData } from '@ibg/utils';
import type { ARB } from '@dyn/arb-dtif';

import { NodeException } from './NodeException';

export class ExportFontAssetException extends NodeException {
	public readonly throwable?: Error;

	constructor(fontInfo: ARB.FontInfo, nodeIds: SceneNode['id'][], throwable?: unknown) {
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
