import type { COMP } from '@dyn/comp-dtif';

import type { TToTransformFill, TToTransformStroke } from '../../FigmaNodeTreeProcessor';
import {
	createDtifStyles,
	mapFigmaBlendModeToDtif,
	mapFigmaTransformToRotation,
	mapFigmaTransformToTranslation
} from '../../utils';

export function transformRectangleNode(
	node: RectangleNode,
	config: TTransformRectangleNodeConfig
): { type: 'Rectangle' } & COMP.RectangleNode {
	const { fills, strokes } = config;

	return {
		type: 'Rectangle',
		visible: node.visible,
		size: [node.height, node.width],
		translation: mapFigmaTransformToTranslation(node.relativeTransform),
		angle: mapFigmaTransformToRotation(node.relativeTransform),
		cornerRadii: [
			node.bottomLeftRadius,
			node.bottomRightRadius,
			node.topLeftRadius,
			node.topRightRadius
		],
		blendMode: mapFigmaBlendModeToDtif(node.blendMode),
		opacity: node.opacity,
		styles: createDtifStyles(fills, strokes)
	};
}

interface TTransformRectangleNodeConfig {
	fills: TToTransformFill[];
	strokes: TToTransformStroke[];
}
