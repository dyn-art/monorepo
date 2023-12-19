import type { TNode } from '@dyn/dtif';

import { InvisibleNodeException, UnsupportedFigmaNodeException } from '../../exceptions';
import type { TToTransformNode } from '../../FigmaNodeTreeProcessor';
import { canBeTransformedToDTIF } from '../../utils';
import { transformFrameNode } from './transform-frame-node';
import { transformGroupNode } from './transform-group-node';
import { transformRectangleNode } from './transform-rectangle-node';
import { transformTextNode } from './transform-text-node';

export async function transformNode(
	toTransformNode: TToTransformNode,
	config: TTransformNodeConfig
): Promise<TNode> {
	const node = toTransformNode.node;

	// Check whether Figma node is supported by DTIF
	if (!canBeTransformedToDTIF(node.type)) {
		throw new UnsupportedFigmaNodeException(node);
	}

	// Check whether node is visible
	if (!node.visible) {
		throw new InvisibleNodeException(node);
	}

	switch (node.type) {
		case 'FRAME':
		case 'COMPONENT':
		case 'INSTANCE':
			return transformFrameNode(node, {
				childrenIds: toTransformNode.childrenIds ?? [],
				paintIds: toTransformNode.paintIds ?? []
			});
		case 'GROUP':
			return transformGroupNode(node, {
				childrenIds: toTransformNode.childrenIds ?? [],
				paintIds: toTransformNode.paintIds ?? []
			});
		case 'TEXT':
			return transformTextNode(node, {
				paintIds: toTransformNode.paintIds ?? [],
				fontIds: toTransformNode.fontIds ?? []
			});
		case 'RECTANGLE':
			return transformRectangleNode(node, { paintIds: toTransformNode.paintIds ?? [] });
		// case 'ELLIPSE':
		// // return transformEllipseNode(node, options);
		// case 'POLYGON':
		// // return transformPolygonNode(node, options);
		// case 'STAR':
		// // return transformStarNode(node, options);
		// case 'LINE':
		// case 'VECTOR':
		// case 'BOOLEAN_OPERATION':
		// // return transformToSVGNode(node, options);
		default:
			throw new UnsupportedFigmaNodeException(node);
	}
}

export interface TTransformNodeConfig {
	includeInvisible: boolean;
}
