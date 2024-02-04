import type { COMP } from '@dyn/dtif';

import { mapFigmaBlendModeToDTIF, mapFigmaTransformToMat3 } from '../../utils';

export function transformStarNode(
	node: StarNode,
	config: TTransformStarNodeConfig
): { type: 'Star' } & COMP.StarNodeBundle {
	const { paintIds } = config;

	return {
		type: 'Star',
		node: {
			node_type: 'Star'
		},
		name: node.name,
		compositionMixin: {
			isLocked: node.locked,
			isVisible: node.visible
		},
		innerRadiusRatio: node.innerRadius,
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

interface TTransformStarNodeConfig {
	paintIds: number[];
}
