import type { COMP } from '@dyn/comp-dtif';

import { mapFigmaRGBToDtif } from '../../utils';

export function transformSolidPaint(paint: SolidPaint): { type: 'Solid' } & COMP.SolidPaint {
	return {
		type: 'Solid',
		color: mapFigmaRGBToDtif(paint.color)
	};
}
