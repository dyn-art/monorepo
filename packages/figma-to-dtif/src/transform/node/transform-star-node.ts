import type { COMP } from '@dyn/dtif-comp';

import type { TToTransformFill, TToTransformStroke } from '../../FigmaNodeTreeProcessor';
import {
	createDtifStyles,
	mapFigmaBlendModeToDtif,
	mapFigmaTransformToRotation,
	mapFigmaTransformToTranslation
} from '../../utils';

export function transformStarNode(
	node: StarNode,
	config: TTransformStarNodeConfig
): { type: 'Star' } & COMP.StarNode {
	const { fills, strokes } = config;

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
		styles: createDtifStyles(fills, strokes)
	};
}

interface TTransformStarNodeConfig {
	fills: TToTransformFill[];
	strokes: TToTransformStroke[];
}
