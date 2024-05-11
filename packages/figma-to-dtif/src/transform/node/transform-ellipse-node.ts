import type { COMP } from '@dyn/comp-dtif';

import type {
	TToTransformEffect,
	TToTransformFill,
	TToTransformStroke
} from '../../FigmaNodeTreeProcessor';
import {
	createDtifStyles,
	mapFigmaBlendModeToDtif,
	mapFigmaElementLayoutToDtif,
	mapFigmaTransformToRotation,
	mapFigmaTransformToTranslation
} from '../../utils';

export function transformEllipseNode(
	node: EllipseNode,
	config: TTransformEllipseNodeConfig
): { type: 'Ellipse' } & COMP.EllipseNode {
	const { fills, strokes, effects, autoLayout } = config;

	return {
		type: 'Ellipse',
		startingAngle: node.arcData.startingAngle,
		endingAngle: node.arcData.endingAngle,
		innerRadiusRatio: node.arcData.innerRadius,
		visible: node.visible,
		size: [node.width, node.height],
		translation: mapFigmaTransformToTranslation(node.relativeTransform),
		rotationDeg: mapFigmaTransformToRotation(node.relativeTransform),
		blendMode: mapFigmaBlendModeToDtif(node.blendMode),
		opacity: node.opacity,
		layoutElement: mapFigmaElementLayoutToDtif(node, autoLayout),
		styles: createDtifStyles(fills, strokes, effects)
	};
}

interface TTransformEllipseNodeConfig {
	fills: TToTransformFill[];
	strokes: TToTransformStroke[];
	effects: TToTransformEffect[];
	autoLayout: boolean;
}
