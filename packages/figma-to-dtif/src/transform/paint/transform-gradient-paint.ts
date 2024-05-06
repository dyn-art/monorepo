import type { COMP } from '@dyn/comp-dtif';

import { UnsupportedFigmaPaintException } from '../../exceptions';
import { mapFigmaRGBToDtif, mapFigmaTransformToMat3 } from '../../utils';

export function transformGradientPaint(
	paint: GradientPaint,
	nodeIds: SceneNode['id'][]
): { type: 'Gradient' } & COMP.GradientPaint {
	return {
		type: 'Gradient',
		variant: mapFigmaGradientTypeToDTIF(paint, nodeIds),
		stops: paint.gradientStops.map((stop) => ({
			color: mapFigmaRGBToDtif(stop.color),
			position: stop.position
		}))
	};
}

function mapFigmaGradientTypeToDTIF(
	paint: GradientPaint,
	nodeIds: SceneNode['id'][]
): COMP.GradientVariant {
	switch (paint.type) {
		case 'GRADIENT_LINEAR':
			return {
				type: 'Linear',
				transform: mapFigmaTransformToMat3(paint.gradientTransform)
			};
		case 'GRADIENT_RADIAL':
			return {
				type: 'Radial',
				transform: mapFigmaTransformToMat3(paint.gradientTransform)
			};
		default:
			throw new UnsupportedFigmaPaintException(paint, nodeIds[0] as any);
	}
}
