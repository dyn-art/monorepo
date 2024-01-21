import type { COMP } from '@dyn/dtif';

import { UnsupportedFigmaPaintException } from '../../exceptions';
import type { TToTransformPaint } from '../../FigmaNodeTreeProcessor';
import {
	transformGradientPaint,
	type TTransformGradientPaintConfig
} from './transform-gradient-paint';
import type { TTransformImagePaintConfig } from './transform-image-paint';
import { transformSolidPaint } from './transform-solid-paint';

export async function transformPaint(
	toTransformPaint: TToTransformPaint,
	config: TTransformPaintConfig
): Promise<COMP.Paint> {
	const paint = toTransformPaint.paint;
	switch (paint.type) {
		case 'SOLID':
			return transformSolidPaint(paint);
		case 'GRADIENT_LINEAR':
		case 'GRADIENT_RADIAL':
		case 'GRADIENT_ANGULAR':
		case 'GRADIENT_DIAMOND':
			return transformGradientPaint(paint, config.gradient);
		// case 'IMAGE':
		default:
			throw new UnsupportedFigmaPaintException(paint, toTransformPaint.nodeIds[0] as any);
	}
}

export interface TTransformPaintConfig {
	gradient: TTransformGradientPaintConfig;
	image: TTransformImagePaintConfig;
}
