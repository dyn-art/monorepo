import type { COMP } from '@dyn/comp-dtif';
import type { TContinuousId } from '@dyn/utils';

import { mapFigmaTransformToMat3 } from '../../utils';

export function transformImagePaint(
	paint: ImagePaint,
	assetId: TContinuousId
): { type: 'Image' } & COMP.ImagePaint {
	return {
		type: 'Image',
		assetId: assetId.toString(),
		scaleMode: resolveScaleMode(paint)
	};
}

function resolveScaleMode(paint: ImagePaint): COMP.ImagePaint['scaleMode'] {
	switch (paint.scaleMode) {
		case 'CROP':
			return {
				type: 'Crop',
				transform: mapFigmaTransformToMat3(
					paint.imageTransform ?? [
						[0, 0, 1],
						[1, 0, 0]
					]
				)
			};
		case 'FILL':
			return {
				type: 'Fill'
			};
		case 'FIT':
			return {
				type: 'Fit'
			};
		case 'TILE':
			return {
				type: 'Tile',
				rotation: paint.rotation ?? 0,
				scalingFactor: paint.scalingFactor ?? 1
			};
	}
}
