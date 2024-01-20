import type { COMP } from '@dyn/dtif';

import { mapFigmaBlendModeToDTIF, mapFigmaRGBToDTIF } from '../../utils';

export function transformSolidPaint(paint: SolidPaint): { type: 'Solid' } & COMP.SolidPaint {
	return {
		type: 'Solid',
		blendMode: mapFigmaBlendModeToDTIF(paint.blendMode),
		color: mapFigmaRGBToDTIF(paint.color),
		opacity: paint.opacity ?? 1,
		isVisible: paint.visible ?? true
	};
}
