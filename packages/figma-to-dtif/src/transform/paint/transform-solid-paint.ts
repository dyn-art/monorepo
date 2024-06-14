import type { CNV } from '@dyn/cnv-dtif';

import { mapFigmaRGBToDtif } from '../../utils';

export function transformSolidPaint(paint: SolidPaint): { type: 'Solid' } & CNV.SolidPaint {
	return {
		type: 'Solid',
		color: mapFigmaRGBToDtif(paint.color)
	};
}
