import type { COMP } from '@dyn/comp-dtif';

import type { TToTransformFill, TToTransformStroke } from '../../FigmaNodeTreeProcessor';
import {
	createDtifStyles,
	mapFigmaBlendModeToDtif,
	mapFigmaTransformToRotation,
	mapFigmaTransformToTranslation
} from '../../utils';

export function transformPolygonNode(
	node: PolygonNode,
	config: TTransformPolygonNodeConfig
): { type: 'Polygon' } & COMP.PolygonNode {
	const { fills, strokes } = config;

	return {
		type: 'Polygon',
		pointCount: node.pointCount,
		visible: node.visible,
		size: [node.height, node.width],
		translation: mapFigmaTransformToTranslation(node.relativeTransform),
		angle: mapFigmaTransformToRotation(node.relativeTransform),
		blendMode: mapFigmaBlendModeToDtif(node.blendMode),
		opacity: node.opacity,
		styles: createDtifStyles(fills, strokes)
	};
}

interface TTransformPolygonNodeConfig {
	fills: TToTransformFill[];
	strokes: TToTransformStroke[];
}
