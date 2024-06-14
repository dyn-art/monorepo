import type { TContinuousId } from '@ibg/utils';
import type { ARB } from '@dyn/arb-dtif';

import { mapFigmaTransformToMat3 } from '../../utils';

export function transformImagePaint(
	paint: ImagePaint,
	imageId: TContinuousId
): { type: 'Image' } & ARB.ImagePaint {
	return {
		type: 'Image',
		imageId: { type: 'ReferenceId', referenceId: `a${imageId}` },
		scaleMode: resolveScaleMode(paint)
	};
}

function resolveScaleMode(paint: ImagePaint): ARB.ImagePaint['scaleMode'] {
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
