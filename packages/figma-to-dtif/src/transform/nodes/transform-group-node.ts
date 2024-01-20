import type { COMP } from '@dyn/dtif';

import { mapFigmaBlendModeToDTIF, mapFigmaTransformToMat3 } from '../../utils';

export function transformGroupNode(
	node: GroupNode,
	config: TTransformGroupNodeConfig
): { type: 'Group' } & COMP.GroupNodeBundle {
	const { childrenIds } = config;

	return {
		type: 'Group',
		node: {
			node_type: 'Group'
		},
		name: node.name,
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
		blendMixin: {
			blendMode: mapFigmaBlendModeToDTIF(node.blendMode),
			opacity: node.opacity,
			isMask: node.isMask
		}
	};
}

interface TTransformGroupNodeConfig {
	childrenIds: number[];
}
