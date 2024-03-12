import type { COMP } from '@dyn/comp-dtif';

import {
	mapFigmaBlendModeToDtif,
	mapFigmaTransformToRotation,
	mapFigmaTransformToTranslation
} from '../../utils';

export function transformGroupNode(
	node: GroupNode,
	config: TTransformGroupNodeConfig
): { type: 'Group' } & COMP.GroupNode {
	const { childrenIds } = config;

	return {
		type: 'Group',
		visible: node.visible,
		children: childrenIds.map((childId) => childId.toString()),
		size: [node.width, node.height],
		translation: mapFigmaTransformToTranslation(node.relativeTransform),
		angle: mapFigmaTransformToRotation(node.relativeTransform),
		blendMode: mapFigmaBlendModeToDtif(node.blendMode),
		opacity: node.opacity
	};
}

interface TTransformGroupNodeConfig {
	childrenIds: number[];
}
