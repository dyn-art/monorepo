import type { COMP } from '@dyn/dtif';
import type { GradientVariant } from '@dyn/dtif/dist/types/gen/bindings';

import type { TExportImageConfig } from '../../types';
import { mapFigmaBlendModeToDTIF, mapFigmaRGBToDTIF, mapFigmaTransformToMat3 } from '../../utils';

export function transformGradientPaint(
	paint: GradientPaint,
	config: TTransformGradientPaintConfig
): ({ type: 'Gradient' } & COMP.GradientPaint) | ({ type: 'Image' } & COMP.ImagePaint) {
	if (config.inline === true) {
		return transformGradientPaintToInline(paint);
	}
	return transformGradientPaintToImage(paint, config.inline.export);
}

function transformGradientPaintToImage(
	paint: GradientPaint,
	exportConfig: TExportImageConfig
): { type: 'Image' } & COMP.ImagePaint {
	// TODO:
	return null as any;
}

function transformGradientPaintToInline(
	paint: GradientPaint
): { type: 'Gradient' } & COMP.GradientPaint {
	return {
		type: 'Gradient',
		variant: mapFigmaGradientTypeToDTIF(paint.type),
		gradientStops: paint.gradientStops.map((stop) => ({
			color: mapFigmaRGBToDTIF(stop.color),
			position: stop.position
		})),
		transform: mapFigmaTransformToMat3(paint.gradientTransform),
		blendMode: mapFigmaBlendModeToDTIF(paint.blendMode),
		opacity: paint.opacity ?? 1,
		isVisible: paint.visible ?? true
	};
}

function mapFigmaGradientTypeToDTIF(variant: GradientPaint['type']): GradientVariant {
	switch (variant) {
		case 'GRADIENT_LINEAR':
			return 'Linear';
		case 'GRADIENT_RADIAL':
			return 'Radial';
		case 'GRADIENT_ANGULAR':
			return 'Angular';
		case 'GRADIENT_DIAMOND':
			return 'Diamond';
	}
}

export interface TTransformGradientPaintConfig {
	inline: true | { export: TExportImageConfig };
}
