import type { TGroupNodeBundle } from '@dyn/dtif';

import { convertFigmaBlendModeToDTIF, convertFigmaTransformToMat3 } from '../../utils';

export function transformGroupNode(
	node: GroupNode,
	config: TTransformGroupNodeConfig
): { type: 'Group' } & TGroupNodeBundle {
	const { childrenIds, paintIds } = config;

	return {
		type: 'Group',
		node: {
			name: node.name,
			node_type: 'Group'
		},
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
		blendMixin: {
			blendMode: convertFigmaBlendModeToDTIF(node.blendMode),
			opacity: node.opacity,
			isMask: node.isMask
		},
		fill: { paintIds }
	};
}

interface TTransformGroupNodeConfig {
	childrenIds: number[];
	paintIds: number[];
}
