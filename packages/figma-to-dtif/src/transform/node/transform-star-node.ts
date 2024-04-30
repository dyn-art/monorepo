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

export function transformStarNode(
	node: StarNode,
	config: TTransformStarNodeConfig
): { type: 'Star' } & COMP.StarNode {
	const { fills, strokes, effects } = config;

	return {
		type: 'Star',
		innerRadiusRatio: node.innerRadius,
		pointCount: node.pointCount,
		visible: node.visible,
		size: [node.width, node.height],
		translation: mapFigmaTransformToTranslation(node.relativeTransform),
		rotationDeg: mapFigmaTransformToRotation(node.relativeTransform),
		blendMode: mapFigmaBlendModeToDtif(node.blendMode),
		opacity: node.opacity,
		layoutElement: { type: 'Absolute', constraints: mapFigmaConstraintsToDtif(node.constraints) },
		styles: createDtifStyles(fills, strokes, effects)
	};
}

interface TTransformStarNodeConfig {
	fills: TToTransformFill[];
	strokes: TToTransformStroke[];
	effects: TToTransformEffect[];
}
