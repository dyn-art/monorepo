import type { COMP } from '@dyn/dtif-comp';

import { UnsupportedFigmaPaintException } from '../../exceptions';
import type { TToTransformPaint } from '../../FigmaNodeTreeProcessor';
import { transformGradientPaint } from './transform-gradient-paint';
import { transformImagePaint } from './transform-image-paint';
import { transformSolidPaint } from './transform-solid-paint';

export async function transformPaint(toTransformPaint: TToTransformPaint): Promise<COMP.Paint> {
	const paint = toTransformPaint.paint;
	switch (paint.type) {
		case 'SOLID':
			return transformSolidPaint(paint);
		case 'GRADIENT_LINEAR':
		case 'GRADIENT_RADIAL':
		case 'GRADIENT_ANGULAR':
		case 'GRADIENT_DIAMOND':
			return transformGradientPaint(paint, toTransformPaint.nodeIds);
		case 'IMAGE':
			return transformImagePaint(paint, paint.assetId);
		default:
			throw new UnsupportedFigmaPaintException(paint, toTransformPaint.nodeIds[0] as any);
	}
}
