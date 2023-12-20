import type { TSolidPaint } from '@dyn/dtif';

import { convertFigmaBlendModeToDTIF, convertFigmaRGBToDTIF } from '../../utils';

export function transformSolidPaint(paint: SolidPaint): { type: 'Solid' } & TSolidPaint {
	return {
		type: 'Solid',
		blendMode: convertFigmaBlendModeToDTIF(paint.blendMode),
		color: convertFigmaRGBToDTIF(paint.color),
		opacity: paint.opacity ?? 1,
		isVisible: paint.visible ?? true
	};
}
