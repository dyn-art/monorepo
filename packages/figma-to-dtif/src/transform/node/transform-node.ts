import type { ARB } from '@dyn/arb-dtif';

import { InvisibleNodeException } from '../../exceptions';
import type { TToTransformNode } from '../../FigmaNodeTreeProcessor';
import type { Transformer } from '../../Transformer';
import type { TFigmaFormat } from '../../types';
import { transformFrameNode } from './transform-frame-node';
import { transformNodeToImage } from './transform-node-to-image';
import { transformShapeNode } from './transform-shape-node';
import { transformTextNode } from './transform-text-node';

export async function transformNode(
	toTransformNode: TToTransformNode,
	cx: Transformer,
	config: TTransformNodeConfig
): Promise<ARB.Node> {
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
					strokes: toTransformNode.strokes,
					effects: toTransformNode.effects,
					autoLayout: toTransformNode.autoLayout
				});
			}
			return transformNodeToImage(toTransformNode.node, cx, {
				format: shouldExportFrame.format
			});
		}
		case 'Text':
			return transformTextNode(toTransformNode.node, toTransformNode.attributes, {
				fills: toTransformNode.fills,
				strokes: toTransformNode.strokes,
				effects: toTransformNode.effects,
				autoLayout: toTransformNode.autoLayout
			});
		case 'Shape':
			return transformShapeNode(toTransformNode);
		default:
			console.warn(
				`To transform node (${toTransformNode.id}) of type '${toTransformNode.node.type}' not supported yet! Trying to export node as PNG.`
			);
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
