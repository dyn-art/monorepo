import type { COMP } from '@dyn/dtif';

import { mapFigmaBlendModeToDTIF, mapFigmaRGBToDTIF } from '../../utils';

export function transformSolidPaint(paint: SolidPaint): { type: 'Solid' } & COMP.SolidPaintBundle {
	return {
		type: 'Solid',
		compositionMixin: {
			isVisible: paint.visible ?? true
		},
		color: mapFigmaRGBToDTIF(paint.color),
		blendMixin: {
			blendMode: mapFigmaBlendModeToDTIF(paint.blendMode),
			opacity: paint.opacity ?? 1
		}
	};
}
