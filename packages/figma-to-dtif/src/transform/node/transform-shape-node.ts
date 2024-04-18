import type { COMP } from '@dyn/dtif-comp';

import { UnsupportedFigmaNodeException } from '../../exceptions';
import type { TToTransformShapeNode } from '../../FigmaNodeTreeProcessor';
import { transformEllipseNode } from './transform-ellipse-node';
import { transformPolygonNode } from './transform-polygon-node';
import { transformRectangleNode } from './transform-rectangle-node';
import { transformStarNode } from './transform-star-node';
import { transformVectorNode } from './transform-vector-node';

export async function transformShapeNode(
	toTransformNode: TToTransformShapeNode
): Promise<COMP.Node> {
	switch (toTransformNode.node.type) {
		case 'RECTANGLE':
			return transformRectangleNode(toTransformNode.node, {
				fills: toTransformNode.fills,
				strokes: toTransformNode.strokes,
				effects: toTransformNode.effects
			});
		case 'ELLIPSE':
			return transformEllipseNode(toTransformNode.node, {
				fills: toTransformNode.fills,
				strokes: toTransformNode.strokes,
				effects: toTransformNode.effects
			});
		case 'POLYGON':
			return transformPolygonNode(toTransformNode.node, {
				fills: toTransformNode.fills,
				strokes: toTransformNode.strokes,
				effects: toTransformNode.effects
			});
		case 'STAR':
			return transformStarNode(toTransformNode.node, {
				fills: toTransformNode.fills,
				strokes: toTransformNode.strokes,
				effects: toTransformNode.effects
			});
		case 'VECTOR':
			return transformVectorNode(toTransformNode.node, {
				fills: toTransformNode.fills,
				strokes: toTransformNode.strokes,
				effects: toTransformNode.effects
			});
		// case 'LINE':
		default:
			throw new UnsupportedFigmaNodeException(toTransformNode.node);
	}
}
