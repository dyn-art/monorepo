import type { COMP } from '@dyn/dtif-comp';

import { InvisibleNodeException } from '../../exceptions';
import type { TToTransformNode } from '../../FigmaNodeTreeProcessor';
import type { Transformer } from '../../Transformer';
import type { TFigmaFormat } from '../../types';
import { transformFrameNode } from './transform-frame-node';
import { transformGroupNode } from './transform-group-node';
import { transformNodeToImage } from './transform-node-to-image';
import { transformShapeNode } from './transform-shape-node';
import { transformTextNode } from './transform-text-node';

export async function transformNode(
	toTransformNode: TToTransformNode,
	cx: Transformer,
	config: TTransformNodeConfig
): Promise<COMP.Node> {
	const { includeInvisible, shouldExportFrame } = config;

	// Check whether node is visible
	if (!toTransformNode.node.visible && !includeInvisible) {
		throw new InvisibleNodeException(toTransformNode.node);
	}

	switch (toTransformNode.type) {
		case 'Frame': {
			if (toTransformNode.isRoot || !shouldExportFrame) {
				return transformFrameNode(toTransformNode.node, {
					childrenIds: toTransformNode.childrenIds,
					fills: toTransformNode.fills,
					strokes: toTransformNode.strokes
				});
			}
			return transformNodeToImage(toTransformNode.node, cx, {
				format: shouldExportFrame.format
			});
		}
		case 'Group':
			return transformGroupNode(toTransformNode.node, {
				childrenIds: toTransformNode.childrenIds
			});
		case 'Text':
			return transformTextNode(toTransformNode.node, toTransformNode.segments, {
				fills: toTransformNode.fills,
				strokes: toTransformNode.strokes
			});
		case 'Shape':
			return transformShapeNode(toTransformNode);
		case 'Uncategorized':
			return transformNodeToImage(toTransformNode.node, cx, {
				format: 'PNG'
			});
	}
}

export interface TTransformNodeConfig {
	includeInvisible: boolean;
	exportContainerNode: FrameNode;
	shouldExportFrame: false | { format: TFigmaFormat };
}
