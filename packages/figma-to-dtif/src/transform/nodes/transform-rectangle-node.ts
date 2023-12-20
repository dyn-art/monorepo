import type { TRectangleNodeBundle } from '@dyn/dtif';

import { convertFigmaBlendModeToDTIF, convertFigmaTransformToMat3 } from '../../utils';

export function transformRectangleNode(
	node: RectangleNode,
	config: TTransformRectangleNodeConfig
): { type: 'Rectangle' } & TRectangleNodeBundle {
	const { paintIds } = config;

	return {
		type: 'Rectangle',
		node: {
			name: node.name,
			node_type: 'Rectangle'
		},
		compositionMixin: {
			isLocked: node.locked,
			isVisible: node.visible
		},
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

interface TTransformRectangleNodeConfig {
	paintIds: number[];
}