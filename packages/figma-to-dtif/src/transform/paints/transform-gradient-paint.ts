import type { COMP } from '@dyn/dtif';

import { mapFigmaBlendModeToDTIF, mapFigmaRGBToDTIF, mapFigmaTransformToMat3 } from '../../utils';

export function transformGradientPaint(
	paint: GradientPaint
): { type: 'Gradient' } & COMP.GradientPaintBundle {
	return {
		type: 'Gradient',
		compositionMixin: {
			isVisible: paint.visible ?? true
		},
		variant: mapFigmaGradientTypeToDTIF(paint.type, paint.gradientTransform),
		gradientStops: paint.gradientStops.map((stop) => ({
			color: mapFigmaRGBToDTIF(stop.color),
			position: stop.position
		})),
		blendMixin: {
			blendMode: mapFigmaBlendModeToDTIF(paint.blendMode),
			opacity: paint.opacity ?? 1
		}
	};
}

function mapFigmaGradientTypeToDTIF(
	variant: GradientPaint['type'],
	transform: Transform
): COMP.GradientPaintVariant {
	switch (variant) {
		case 'GRADIENT_LINEAR':
			return {
				type: 'Linear',
				transform: { type: 'Basic', transform: mapFigmaTransformToMat3(transform) }
			};
		case 'GRADIENT_RADIAL':
			return {
				type: 'Radial',
				transform: { type: 'Basic', transform: mapFigmaTransformToMat3(transform) }
			};
		default:
			throw Error('todo');
	}
}
