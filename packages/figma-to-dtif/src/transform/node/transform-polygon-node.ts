import type { COMP } from '@dyn/dtif-comp';

import type {
	TToTransformEffect,
	TToTransformFill,
	TToTransformStroke
} from '../../FigmaNodeTreeProcessor';
import {
	createDtifStyles,
	mapFigmaBlendModeToDtif,
	mapFigmaConstraintToDtif,
	mapFigmaTransformToRotation,
	mapFigmaTransformToTranslation
} from '../../utils';

export function transformPolygonNode(
	node: PolygonNode,
	config: TTransformPolygonNodeConfig
): { type: 'Polygon' } & COMP.PolygonNode {
	const { fills, strokes, effects } = config;

	return {
		type: 'Polygon',
		pointCount: node.pointCount,
		visible: node.visible,
		size: [node.width, node.height],
		translation: mapFigmaTransformToTranslation(node.relativeTransform),
		rotationDeg: mapFigmaTransformToRotation(node.relativeTransform),
		blendMode: mapFigmaBlendModeToDtif(node.blendMode),
		opacity: node.opacity,
		alignSelf: mapFigmaConstraintToDtif(node.constraints.horizontal),
		justifySelf: mapFigmaConstraintToDtif(node.constraints.vertical),
		styles: createDtifStyles(fills, strokes, effects)
	};
}

interface TTransformPolygonNodeConfig {
	fills: TToTransformFill[];
	strokes: TToTransformStroke[];
	effects: TToTransformEffect[];
}
