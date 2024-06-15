import type { ARB } from '@dyn/arb-dtif';

import { mapFigmaRGBToDtif } from '../../utils';

export function transformSolidPaint(paint: SolidPaint): { type: 'Solid' } & ARB.SolidPaint {
	return {
		type: 'Solid',
		color: mapFigmaRGBToDtif(paint.color)
	};
}
