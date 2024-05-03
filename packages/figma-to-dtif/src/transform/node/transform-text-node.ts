import type { COMP } from '@dyn/dtif-comp';
import { calculateBytes } from '@dyn/utils';

import type {
	TToTransformEffect,
	TToTransformFill,
	TToTransformStroke,
	TToTransformTextNode
} from '../../FigmaNodeTreeProcessor';
import {
	createDtifStyles,
	mapFigmaBlendModeToDtif,
	mapFigmaConstraintsToDtif,
	mapFigmaTextAlignHorizontalToDtif,
	mapFigmaTextAlignVerticalToDtif,
	mapFigmaTransformToRotation,
	mapFigmaTransformToTranslation
} from '../../utils';

export function transformTextNode(
	node: TextNode,
	attributes: TToTransformTextNode['attributes'],
	config: TTransformTextNodeNodeConfig
): { type: 'Text' } & COMP.TextNode {
	const { fills, strokes, effects, autoLayout } = config;

	let nextStart = 0;

	return {
		type: 'Text',
		text: node.characters,
		attributes: attributes.map((attribute) => {
			// Figma works on char and not byte level for their ranges.
			// e.g. for "·" the end is 1 although it should be 2 on byte level.
			// Thus we need to map from char to byte level.
			const currentStart = nextStart === 0 ? attribute.start : nextStart;
			nextStart = nextStart + calculateBytes(attribute.characters);

			return {
				start: currentStart,
				end: nextStart,
				attributes: {
					fontFamily: attribute.fontInfo.family,
					fontStretch: attribute.fontInfo.variant.stretch,
					fontStyle: attribute.fontInfo.variant.style,
					fontWeight: attribute.fontInfo.variant.weight,
					fontSize: attribute.fontSize,
					letterSpacing:
						attribute.letterSpacing.unit === 'PERCENT'
							? {
									type: 'Em',
									value: attribute.letterSpacing.value / 100
								}
							: { type: 'Abs', value: attribute.letterSpacing.value },
					lineHeight:
						// eslint-disable-next-line no-nested-ternary -- Ok here
						attribute.lineHeight.unit === 'AUTO'
							? undefined
							: attribute.lineHeight.unit === 'PERCENT'
								? {
										type: 'Em',
										value: attribute.lineHeight.value / 100
									}
								: {
										type: 'Abs',
										value: attribute.lineHeight.value
									}
				}
			};
		}),
		lineWrap: 'Word',
		horizontalTextAlignment: mapFigmaTextAlignHorizontalToDtif(node.textAlignHorizontal),
		verticalTextAlignment: mapFigmaTextAlignVerticalToDtif(node.textAlignVertical),
		visible: node.visible,
		size: [node.width, node.height],
		translation: mapFigmaTransformToTranslation(node.relativeTransform),
		rotationDeg: mapFigmaTransformToRotation(node.relativeTransform),
		blendMode: mapFigmaBlendModeToDtif(node.blendMode),
		opacity: node.opacity,
		layoutElement: autoLayout
			? { type: 'Static' }
			: { type: 'Absolute', constraints: mapFigmaConstraintsToDtif(node.constraints) },
		styles: createDtifStyles(fills, strokes, effects)
	};
}

interface TTransformTextNodeNodeConfig {
	fills: TToTransformFill[];
	strokes: TToTransformStroke[];
	effects: TToTransformEffect[];
	autoLayout: boolean;
}
