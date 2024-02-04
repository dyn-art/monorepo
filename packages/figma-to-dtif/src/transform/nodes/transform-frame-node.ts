import type { COMP } from '@dyn/dtif';

import { mapFigmaBlendModeToDTIF, mapFigmaTransformToMat3 } from '../../utils';

export function transformFrameNode(
	node: FrameNode | ComponentNode | InstanceNode,
	config: TTransformFrameNodeConfig
): { type: 'Frame' } & COMP.FrameNodeBundle {
	const { childrenIds, paintIds } = config;

	return {
		type: 'Frame',
		node: {
			node_type: 'Frame'
		},
		name: node.name,
		clipContent: node.clipsContent,
		compositionMixin: {
			isLocked: node.locked,
			isVisible: node.visible
		},
		children: childrenIds,
		dimension: {
			height: node.height,
			width: node.width
		},
		relativeTransform: mapFigmaTransformToMat3(node.relativeTransform),
		rectangleCornerMixin: {
			bottomLeftRadius: node.bottomLeftRadius,
			bottomRightRadius: node.bottomRightRadius,
			topLeftRadius: node.topLeftRadius,
			topRightRadius: node.topRightRadius
		},
		blendMixin: {
			blendMode: mapFigmaBlendModeToDTIF(node.blendMode),
			opacity: node.opacity,
			isMask: node.isMask
		},
		fill: { paintIds }
	};
}

interface TTransformFrameNodeConfig {
	childrenIds: number[];
	paintIds: number[];
}
