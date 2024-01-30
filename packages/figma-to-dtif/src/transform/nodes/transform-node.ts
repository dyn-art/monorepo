import type { COMP } from '@dyn/dtif';

import { InvisibleNodeException } from '../../exceptions';
import type { TToTransformNode } from '../../FigmaNodeTreeProcessor';
import type { Transformer } from '../../Transformer';
import { transformFrameNode } from './transform-frame-node';
import { transformGroupNode } from './transform-group-node';
import { transformShapeNode } from './transform-shape-node';
import { transformTextNode } from './transform-text-node';
import { transformToImageNode } from './transform-to-image-node';

export async function transformNode(
	toTransformNode: TToTransformNode,
	cx: Transformer,
	config: TTransformNodeConfig
): Promise<COMP.NodeBundle> {
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
					paintIds: toTransformNode.paintIds
				});
			}
			return transformToImageNode(toTransformNode.node, cx, {
				contentType: shouldExportFrame.contentType
			});
		}
		case 'Group':
			return transformGroupNode(toTransformNode.node, {
				childrenIds: toTransformNode.childrenIds
			});
		case 'Text':
			return transformTextNode(toTransformNode.node, toTransformNode.segments, {
				paintIds: toTransformNode.paintIds
			});
		case 'Shape':
			return transformShapeNode(toTransformNode);
	}
}

export interface TTransformNodeConfig {
	includeInvisible: boolean;
	exportContainerNode: FrameNode;
	shouldExportFrame: false | { contentType: COMP.ContentType };
}
