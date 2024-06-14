import type { ARB } from '@dyn/arb-dtif';

import type {
	TToTransformEffect,
	TToTransformFill,
	TToTransformStroke
} from '../../FigmaNodeTreeProcessor';
import {
	createDtifStyles,
	mapFigmaBlendModeToDtif,
	mapFigmaElementLayoutToDtif,
	mapFigmaTransformToRotation,
	mapFigmaTransformToTranslation
} from '../../utils';

export function transformStarNode(
	node: StarNode,
	config: TTransformStarNodeConfig
): { type: 'Star' } & ARB.StarNode {
	const { fills, strokes, effects, autoLayout } = config;

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
		layoutElement: mapFigmaElementLayoutToDtif(node, autoLayout),
		styles: createDtifStyles(fills, strokes, effects)
	};
}

interface TTransformStarNodeConfig {
	fills: TToTransformFill[];
	strokes: TToTransformStroke[];
	effects: TToTransformEffect[];
	autoLayout: boolean;
}
