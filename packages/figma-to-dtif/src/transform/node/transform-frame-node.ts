import type { COMP } from '@dyn/dtif-comp';

import type {
	TToTransformEffect,
	TToTransformFill,
	TToTransformStroke
} from '../../FigmaNodeTreeProcessor';
import {
	createDtifStyles,
	mapFigmaBlendModeToDtif,
	mapFigmaConstraintsToDtif,
	mapFigmaTransformToRotation,
	mapFigmaTransformToTranslation
} from '../../utils';

export function transformFrameNode(
	node: FrameNode | ComponentNode | InstanceNode,
	config: TTransformFrameNodeConfig
): { type: 'Frame' } & COMP.FrameNode {
	const { childrenIds, fills, strokes, effects } = config;

	return {
		type: 'Frame',
		clipContent: node.clipsContent,
		visible: node.visible,
		children: childrenIds.map((childId) => childId.toString()),
		size: [node.width, node.height],
		translation: mapFigmaTransformToTranslation(node.relativeTransform),
		rotationDeg: mapFigmaTransformToRotation(node.relativeTransform),
		cornerRadii: [
			node.topLeftRadius,
			node.topRightRadius,
			node.bottomRightRadius,
			node.bottomLeftRadius
		],
		blendMode: mapFigmaBlendModeToDtif(node.blendMode),
		opacity: node.opacity,
		constraints: mapFigmaConstraintsToDtif(node.constraints),
		styles: createDtifStyles(fills, strokes, effects)
	};
}

interface TTransformFrameNodeConfig {
	childrenIds: number[];
	fills: TToTransformFill[];
	strokes: TToTransformStroke[];
	effects: TToTransformEffect[];
}
