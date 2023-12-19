import { TNode } from '@dyn/dtif';

import { UnsupportedFigmaNodeException } from '../../exceptions';
import { TToTransformShapeNode } from '../../FigmaNodeTreeProcessor';
import { transformRectangleNode } from './transform-rectangle-node';

export async function transformShapeNode(toTransformNode: TToTransformShapeNode): Promise<TNode> {
	switch (toTransformNode.node.type) {
		case 'RECTANGLE':
			return transformRectangleNode(toTransformNode.node, { paintIds: toTransformNode.paintIds });
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
			throw new UnsupportedFigmaNodeException(toTransformNode.node);
	}
}
