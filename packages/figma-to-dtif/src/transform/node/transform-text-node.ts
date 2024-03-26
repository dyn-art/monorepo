import type { COMP } from '@dyn/dtif-comp';

import type {
	TToTransformFill,
	TToTransformStroke,
	TToTransformTextNode
} from '../../FigmaNodeTreeProcessor';
import {
	createDtifStyles,
	mapFigmaBlendModeToDtif,
	mapFigmaTransformToRotation,
	mapFigmaTransformToTranslation
} from '../../utils';

export function transformTextNode(
	node: TextNode,
	attributes: TToTransformTextNode['attributes'],
	config: TTransformTextNodeNodeConfig
): { type: 'Text' } & COMP.TextNode {
	const { fills, strokes } = config;

	return {
		type: 'Text',
		text: node.characters,
		attributes: attributes.map((attribute) => ({
			start: attribute.start,
			end: attribute.end,
			attributes: {
				fontFamily: attribute.fontInfo.family,
				fontStretch: attribute.fontInfo.variant.stretch,
				fontStyle: attribute.fontInfo.variant.style,
				fontWeight: attribute.fontInfo.variant.weight,
				fontSize: attribute.fontSize
			}
		})),
		lineWrap: 'Word',
		visible: node.visible,
		size: [node.width, node.height],
		translation: mapFigmaTransformToTranslation(node.relativeTransform),
		rotationDeg: mapFigmaTransformToRotation(node.relativeTransform),
		blendMode: mapFigmaBlendModeToDtif(node.blendMode),
		opacity: node.opacity,
		styles: createDtifStyles(fills, strokes)
	};
}

interface TTransformTextNodeNodeConfig {
	fills: TToTransformFill[];
	strokes: TToTransformStroke[];
}
