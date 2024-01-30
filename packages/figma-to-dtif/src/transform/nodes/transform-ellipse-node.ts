import type { COMP } from '@dyn/dtif';

import { mapFigmaBlendModeToDTIF, mapFigmaTransformToMat3 } from '../../utils';

export function transformEllipseNode(
	node: EllipseNode,
	config: TTransformEllipseNodeConfig
): { type: 'Ellipse' } & COMP.EllipseNodeBundle {
	const { paintIds } = config;

	return {
		type: 'Ellipse',
		node: {
			node_type: 'Ellipse'
		},
		name: node.name,
		compositionMixin: {
			isLocked: node.locked,
			isVisible: node.visible
		},
		arcData: {
			startingAngle: node.arcData.startingAngle,
			endingAngle: node.arcData.endingAngle,
			innerRadiusRatio: node.arcData.innerRadius
		},
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

interface TTransformEllipseNodeConfig {
	paintIds: number[];
}
