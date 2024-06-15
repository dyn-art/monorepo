import type { ARB } from '@dyn/arb-dtif';

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

export function transformVectorNode(
	node: VectorNode,
	config: TTransformVectorNodeConfig
): { type: 'Vector' } & ARB.VectorNode {
	const { fills, strokes, effects, autoLayout } = config;

	return {
		type: 'Vector',
		path: node.vectorPaths[0]?.data ?? '', // TODO: Support multi path nodes
		windingRule: node.vectorPaths[0]?.windingRule === 'EVENODD' ? 'Evenodd' : 'Nonzero',
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

interface TTransformVectorNodeConfig {
	fills: TToTransformFill[];
	strokes: TToTransformStroke[];
	effects: TToTransformEffect[];
	autoLayout: boolean;
}
