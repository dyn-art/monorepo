import type { COMP } from '@dyn/dtif-comp';

import type {
	TToTransformEffect,
	TToTransformFill,
	TToTransformStroke
} from '../../FigmaNodeTreeProcessor';
import {
	createDtifStyles,
	mapFigmaBlendModeToDtif,
	mapFigmaConstraintsToDtif,
	mapFigmaTransformToRotation,
	mapFigmaTransformToTranslation
} from '../../utils';

export function transformRectangleNode(
	node: RectangleNode,
	config: TTransformRectangleNodeConfig
): { type: 'Rectangle' } & COMP.RectangleNode {
	const { fills, strokes, effects } = config;

	return {
		type: 'Rectangle',
		visible: node.visible,
		size: [node.width, node.height],
		translation: mapFigmaTransformToTranslation(node.relativeTransform),
		rotationDeg: mapFigmaTransformToRotation(node.relativeTransform),
		cornerRadii: [
			node.topLeftRadius,
			node.topRightRadius,
			node.bottomRightRadius,
			node.bottomLeftRadius
		],
		blendMode: mapFigmaBlendModeToDtif(node.blendMode),
		opacity: node.opacity,
		layoutElement: { type: 'Absolute', constraints: mapFigmaConstraintsToDtif(node.constraints) },
		styles: createDtifStyles(fills, strokes, effects)
	};
}

interface TTransformRectangleNodeConfig {
	fills: TToTransformFill[];
	strokes: TToTransformStroke[];
	effects: TToTransformEffect[];
}
