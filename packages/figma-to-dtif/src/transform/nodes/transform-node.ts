import type { COMP } from '@dyn/dtif';

import { InvisibleNodeException } from '../../exceptions';
import type { TToTransformNode } from '../../FigmaNodeTreeProcessor';
import { transformFrameNode } from './transform-frame-node';
import { transformGroupNode } from './transform-group-node';
import { transformShapeNode } from './transform-shape-node';
import { transformTextNode } from './transform-text-node';

export async function transformNode(
	toTransformNode: TToTransformNode,
	config: TTransformNodeConfig
): Promise<COMP.NodeBundle> {
	// Check whether node is visible
	if (!toTransformNode.node.visible && !config.includeInvisible) {
		throw new InvisibleNodeException(toTransformNode.node);
	}

	switch (toTransformNode.type) {
		case 'Frame':
			return transformFrameNode(toTransformNode.node, {
				childrenIds: toTransformNode.childrenIds,
				paintIds: toTransformNode.paintIds
			});
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
}
