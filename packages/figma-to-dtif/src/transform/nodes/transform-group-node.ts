import type { COMP } from '@dyn/dtif';

import { convertFigmaBlendModeToDTIF, convertFigmaTransformToMat3 } from '../../utils';

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
		relativeTransform: convertFigmaTransformToMat3(node.relativeTransform),
		blendMixin: {
			blendMode: convertFigmaBlendModeToDTIF(node.blendMode),
			opacity: node.opacity,
			isMask: node.isMask
		}
	};
}

interface TTransformGroupNodeConfig {
	childrenIds: number[];
}
