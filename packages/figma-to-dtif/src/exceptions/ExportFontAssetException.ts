import { extractErrorData } from '@ibg/utils';
import type { CNV } from '@dyn/cnv-dtif';

import { NodeException } from './NodeException';

export class ExportFontAssetException extends NodeException {
	public readonly throwable?: Error;

	constructor(fontInfo: CNV.FontInfo, nodeIds: SceneNode['id'][], throwable?: unknown) {
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
