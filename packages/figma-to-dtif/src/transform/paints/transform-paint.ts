import type { COMP } from '@dyn/dtif';

import { UnsupportedFigmaPaintException } from '../../exceptions';
import type { TToTransformPaint } from '../../FigmaNodeTreeProcessor';
import { transformGradientPaint } from './transform-gradient-paint';
import { transformImagePaint, type TTransformImagePaintConfig } from './transform-image-paint';
import { transformSolidPaint } from './transform-solid-paint';

export async function transformPaint(
	toTransformPaint: TToTransformPaint,
	config: TTransformPaintConfig
): Promise<COMP.PaintBundle> {
	const paint = toTransformPaint.paint;
	switch (paint.type) {
		case 'SOLID':
			return transformSolidPaint(paint);
		case 'GRADIENT_LINEAR':
		case 'GRADIENT_RADIAL':
		case 'GRADIENT_ANGULAR':
		case 'GRADIENT_DIAMOND':
			return transformGradientPaint(paint);
		case 'IMAGE':
			return transformImagePaint(paint, toTransformPaint.nodeIds, config.image);
		default:
			throw new UnsupportedFigmaPaintException(paint, toTransformPaint.nodeIds[0] as any);
	}
}

export interface TTransformPaintConfig {
	image: TTransformImagePaintConfig;
}
