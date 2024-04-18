import type { COMP } from '@dyn/dtif-comp';
import { notEmpty } from '@dyn/utils';

import type {
	TToTransformEffect,
	TToTransformFill,
	TToTransformStroke
} from '../../FigmaNodeTreeProcessor';
import { mapFigmaRGBToDtif } from '../mapper';
import { mapFigmaBlendModeToDtif } from '../mapper/map-figma-blend-mode-to-dtif';

export function createDtifStyles(
	fills: TToTransformFill[],
	strokes: TToTransformStroke[],
	effects: TToTransformEffect[]
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
		)
		.concat(
			effects
				// eslint-disable-next-line array-callback-return -- All cases handled in switch
				.map((effect) => {
					switch (effect.variant.type) {
						case 'DROP_SHADOW': {
							const dropShadow = effect.variant;
							return {
								type: 'DropShadow',
								color: mapFigmaRGBToDtif(dropShadow.color),
								position: [dropShadow.offset.x, dropShadow.offset.y],
								blur: dropShadow.radius,
								spread: dropShadow.spread,
								visible: dropShadow.visible,
								blendMode: mapFigmaBlendModeToDtif(dropShadow.blendMode),
								opacity: dropShadow.color.a
							} as COMP.Style;
						}
						case 'INNER_SHADOW':
						case 'LAYER_BLUR':
						case 'BACKGROUND_BLUR':
							return undefined; // TODO
					}
				})
				.filter(notEmpty)
		);
}
