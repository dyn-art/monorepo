import type { COMP } from '@dyn/comp-dtif';

import type { TToTransformFill, TToTransformStroke } from '../../FigmaNodeTreeProcessor';
import { mapFigmaBlendModeToDtif } from '../mapper/map-figma-blend-mode-to-dtif';

export function createDtifStyles(
	fills: TToTransformFill[],
	strokes: TToTransformStroke[]
): COMP.Style[] {
	return fills
		.map(
			(fill) =>
				({
					type: 'Fill',
					paintId: fill.paintId.toString(),
					blendMode: mapFigmaBlendModeToDtif(fill.blendMode),
					opacity: fill.opacity,
					visible: fill.visible
				}) as COMP.Style
		)
		.concat(
			strokes.map(
				(stroke) =>
					({
						type: 'Stroke',
						width: stroke.width,
						paintId: stroke.paintId.toString(),
						blendMode: mapFigmaBlendModeToDtif(stroke.blendMode),
						opacity: stroke.opacity,
						visible: stroke.visible
					}) as COMP.Style
			)
		);
}
