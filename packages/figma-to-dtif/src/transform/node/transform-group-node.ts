import type { COMP } from '@dyn/dtif-comp';

import { mapFigmaBlendModeToDtif } from '../../utils';

export function transformGroupNode(
	node: GroupNode,
	config: TTransformGroupNodeConfig
): { type: 'Group' } & COMP.GroupNode {
	const { childrenIds } = config;

	return {
		type: 'Group',
		visible: node.visible,
		children: childrenIds.map((childId) => childId.toString()),
		blendMode: mapFigmaBlendModeToDtif(node.blendMode),
		opacity: node.opacity
	};
}

interface TTransformGroupNodeConfig {
	childrenIds: number[];
}
