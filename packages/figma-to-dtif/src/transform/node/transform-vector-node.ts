import type { COMP } from '@dyn/dtif-comp';

import type { TToTransformFill, TToTransformStroke } from '../../FigmaNodeTreeProcessor';
import {
	createDtifStyles,
	mapFigmaBlendModeToDtif,
	mapFigmaTransformToRotation,
	mapFigmaTransformToTranslation
} from '../../utils';

export function transformVectorNode(
	node: VectorNode,
	config: TTransformVectorNodeConfig
): { type: 'Vector' } & COMP.VectorNode {
	const { fills, strokes } = config;

	return {
		type: 'Vector',
		path: node.vectorPaths[0]?.data ?? '', // TODO: Support multi path nodes
		visible: node.visible,
		size: [node.width, node.height],
		translation: mapFigmaTransformToTranslation(node.relativeTransform),
		rotationDeg: mapFigmaTransformToRotation(node.relativeTransform),
		blendMode: mapFigmaBlendModeToDtif(node.blendMode),
		opacity: node.opacity,
		styles: createDtifStyles(fills, strokes)
	};
}

interface TTransformVectorNodeConfig {
	fills: TToTransformFill[];
	strokes: TToTransformStroke[];
}
