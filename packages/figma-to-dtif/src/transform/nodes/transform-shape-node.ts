import type { COMP } from '@dyn/dtif';

import { UnsupportedFigmaNodeException } from '../../exceptions';
import type { TToTransformShapeNode } from '../../FigmaNodeTreeProcessor';
import { transformEllipseNode } from './transform-ellipse-node';
import { transformPolygonNode } from './transform-polygon-node';
import { transformRectangleNode } from './transform-rectangle-node';
import { transformStarNode } from './transform-star-node';
import { transformVectorNode } from './transform-vector-node';

export async function transformShapeNode(
	toTransformNode: TToTransformShapeNode
): Promise<COMP.NodeBundle> {
	switch (toTransformNode.node.type) {
		case 'RECTANGLE':
			return transformRectangleNode(toTransformNode.node, { paintIds: toTransformNode.paintIds });
		case 'ELLIPSE':
			return transformEllipseNode(toTransformNode.node, { paintIds: toTransformNode.paintIds });
		case 'POLYGON':
			return transformPolygonNode(toTransformNode.node, { paintIds: toTransformNode.paintIds });
		case 'STAR':
			return transformStarNode(toTransformNode.node, { paintIds: toTransformNode.paintIds });
		case 'VECTOR':
			return transformVectorNode(toTransformNode.node, { paintIds: toTransformNode.paintIds });
		// case 'LINE':
		default:
			throw new UnsupportedFigmaNodeException(toTransformNode.node);
	}
}
