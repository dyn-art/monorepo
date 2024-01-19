import type { COMP } from '@dyn/dtif';

import { convertFigmaBlendModeToDTIF, convertFigmaRGBToDTIF } from '../../utils';

export function transformSolidPaint(paint: SolidPaint): { type: 'Solid' } & COMP.SolidPaint {
	return {
		type: 'Solid',
		blendMode: convertFigmaBlendModeToDTIF(paint.blendMode),
		color: convertFigmaRGBToDTIF(paint.color),
		opacity: paint.opacity ?? 1,
		isVisible: paint.visible ?? true
	};
}
