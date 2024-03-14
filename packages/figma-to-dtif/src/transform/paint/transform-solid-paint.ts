import type { COMP } from '@dyn/dtif-comp';

import { mapFigmaRGBToDtif } from '../../utils';

export function transformSolidPaint(paint: SolidPaint): { type: 'Solid' } & COMP.SolidPaint {
	return {
		type: 'Solid',
		color: mapFigmaRGBToDtif(paint.color)
	};
}
