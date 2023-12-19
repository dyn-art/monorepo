import type { TPaint } from '@dyn/dtif';

import { UnsupportedFigmaPaintException } from '../../exceptions';
import type { TToTransformPaint } from '../../FigmaNodeTreeProcessor';
import type { TExportOptions } from '../../types';
import { transformSolidPaint } from './transform-solid-paint';

export async function transformPaint(
	toTransformPaint: TToTransformPaint,
	config: TTransformPaintConfig
): Promise<TPaint> {
	const paint = toTransformPaint.paint;
	switch (paint.type) {
		case 'SOLID':
			return transformSolidPaint(paint);
		// case 'GRADIENT_LINEAR':
		// case 'GRADIENT_RADIAL':
		// case 'GRADIENT_ANGULAR':
		// case 'GRADIENT_DIAMOND':
		// case 'IMAGE':
		default:
			throw new UnsupportedFigmaPaintException(paint, toTransformPaint.nodeIds[0] as any);
	}
}

export interface TTransformPaintConfig {
	imageExportOptions: TExportOptions;
	gradientExportOptions: TExportOptions;
}
