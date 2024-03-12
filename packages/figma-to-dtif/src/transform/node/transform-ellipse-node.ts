import type { COMP } from '@dyn/comp-dtif';

import type { TToTransformFill, TToTransformStroke } from '../../FigmaNodeTreeProcessor';
import {
	createDtifStyles,
	mapFigmaBlendModeToDtif,
	mapFigmaTransformToRotation,
	mapFigmaTransformToTranslation
} from '../../utils';

export function transformEllipseNode(
	node: EllipseNode,
	config: TTransformEllipseNodeConfig
): { type: 'Ellipse' } & COMP.EllipseNode {
	const { fills, strokes } = config;

	return {
		type: 'Ellipse',
		startingAngle: node.arcData.startingAngle,
		endingAngle: node.arcData.endingAngle,
		innerRadiusRatio: node.arcData.innerRadius,
		visible: node.visible,
		size: [node.width, node.height],
		translation: mapFigmaTransformToTranslation(node.relativeTransform),
		angle: mapFigmaTransformToRotation(node.relativeTransform),
		blendMode: mapFigmaBlendModeToDtif(node.blendMode),
		opacity: node.opacity,
		styles: createDtifStyles(fills, strokes)
	};
}

interface TTransformEllipseNodeConfig {
	fills: TToTransformFill[];
	strokes: TToTransformStroke[];
}
