import type { CNV } from '@dyn/cnv-dtif';

import type {
	TToTransformEffect,
	TToTransformFill,
	TToTransformStroke
} from '../../FigmaNodeTreeProcessor';
import {
	createDtifStyles,
	mapFigmaBlendModeToDtif,
	mapFigmaElementLayoutToDtif,
	mapFigmaParentLayoutToDtif,
	mapFigmaTransformToRotation,
	mapFigmaTransformToTranslation
} from '../../utils';

export function transformFrameNode(
	node: FrameNode | ComponentNode | InstanceNode,
	config: TTransformFrameNodeConfig
): { type: 'Frame' } & CNV.FrameNode {
	const { childrenIds, fills, strokes, effects, autoLayout } = config;

	return {
		type: 'Frame',
		clipContent: node.clipsContent,
		layoutParent: mapFigmaParentLayoutToDtif(node),
		visible: node.visible,
		children: childrenIds.map((childId) => ({ type: 'ReferenceId', referenceId: `n${childId}` })),
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
		layoutElement: mapFigmaElementLayoutToDtif(node, autoLayout),
		styles: createDtifStyles(fills, strokes, effects)
	};
}

interface TTransformFrameNodeConfig {
	childrenIds: number[];
	fills: TToTransformFill[];
	strokes: TToTransformStroke[];
	effects: TToTransformEffect[];
	autoLayout: boolean;
}
