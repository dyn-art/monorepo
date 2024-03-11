import type { COMP } from '@dyn/comp-dtif';

import type { TToTransformFill, TToTransformStroke } from '../../FigmaNodeTreeProcessor';
import {
	createDtifStyles,
	mapFigmaBlendModeToDtif,
	mapFigmaTransformToRotation,
	mapFigmaTransformToTranslation
} from '../../utils';

export function transformFrameNode(
	node: FrameNode | ComponentNode | InstanceNode,
	config: TTransformFrameNodeConfig
): { type: 'Frame' } & COMP.FrameNode {
	const { childrenIds, fills, strokes } = config;

	return {
		type: 'Frame',
		clipContent: node.clipsContent,
		visible: node.visible,
		children: childrenIds.map((childId) => childId.toString()),
		size: [node.height, node.width],
		translation: mapFigmaTransformToTranslation(node.relativeTransform),
		angle: mapFigmaTransformToRotation(node.relativeTransform),
		cornerRadii: [
			node.bottomLeftRadius,
			node.bottomRightRadius,
			node.topLeftRadius,
			node.topRightRadius
		],
		blendMode: mapFigmaBlendModeToDtif(node.blendMode),
		opacity: node.opacity,
		styles: createDtifStyles(fills, strokes)
	};
}

interface TTransformFrameNodeConfig {
	childrenIds: number[];
	fills: TToTransformFill[];
	strokes: TToTransformStroke[];
}
