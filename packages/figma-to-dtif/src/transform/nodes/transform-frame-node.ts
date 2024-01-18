import type { TFrameNodeBundle } from '@dyn/dtif';

import { convertFigmaBlendModeToDTIF, convertFigmaTransformToMat3 } from '../../utils';

export function transformFrameNode(
	node: FrameNode | ComponentNode | InstanceNode,
	config: TTransformFrameNodeConfig
): { type: 'Frame' } & TFrameNodeBundle {
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
		relativeTransform: convertFigmaTransformToMat3(node.relativeTransform),
		rectangleCornerMixin: {
			bottomLeftRadius: node.bottomLeftRadius,
			bottomRightRadius: node.bottomRightRadius,
			topLeftRadius: node.topLeftRadius,
			topRightRadius: node.topRightRadius
		},
		blendMixin: {
			blendMode: convertFigmaBlendModeToDTIF(node.blendMode),
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
