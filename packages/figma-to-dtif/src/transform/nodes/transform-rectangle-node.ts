import type { COMP } from '@dyn/dtif';

import { mapFigmaBlendModeToDTIF, mapFigmaTransformToMat3 } from '../../utils';

export function transformRectangleNode(
	node: RectangleNode,
	config: TTransformRectangleNodeConfig
): { type: 'Rectangle' } & COMP.RectangleNodeBundle {
	const { paintIds } = config;

	return {
		type: 'Rectangle',
		node: {
			node_type: 'Rectangle'
		},
		name: node.name,
		compositionMixin: {
			isLocked: node.locked,
			isVisible: node.visible
		},
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

interface TTransformRectangleNodeConfig {
	paintIds: number[];
}
