import type { COMP } from '@dyn/dtif';

import { mapFigmaBlendModeToDTIF, mapFigmaTransformToMat3 } from '../../utils';

export function transformPolygonNode(
	node: PolygonNode,
	config: TTransformPolygonNodeConfig
): { type: 'Polygon' } & COMP.PolygonNodeBundle {
	const { paintIds } = config;

	return {
		type: 'Polygon',
		node: {
			node_type: 'Polygon'
		},
		name: node.name,
		compositionMixin: {
			isLocked: node.locked,
			isVisible: node.visible
		},
		pointCount: node.pointCount,
		dimension: {
			height: node.height,
			width: node.width
		},
		relativeTransform: mapFigmaTransformToMat3(node.relativeTransform),
		blendMixin: {
			blendMode: mapFigmaBlendModeToDTIF(node.blendMode),
			opacity: node.opacity,
			isMask: node.isMask
		},
		fill: { paintIds }
	};
}

interface TTransformPolygonNodeConfig {
	paintIds: number[];
}
