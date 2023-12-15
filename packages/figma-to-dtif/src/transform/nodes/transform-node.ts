import type { TNode } from '@dyn/dtif';

import { InvisibleNodeException, UnsupportedFigmaNodeException } from '../../exceptions';
import type { TToTransformNode } from '../../Transformer';
import { canBeTransformedToDTIF } from '../../utils';

export async function transformNode(toTransformNode: TToTransformNode): Promise<TNode> {
	const node = toTransformNode.node;

	// Check whether Figma node is supported by DTIF
	if (!canBeTransformedToDTIF(node.type)) {
		throw new UnsupportedFigmaNodeException(node);
	}

	// Check whether node is visible
	if (!node.visible) {
		throw new InvisibleNodeException(node);
	}

	// switch (node.type) {
	// 	case 'FRAME':
	// 	case 'COMPONENT':
	// 	case 'INSTANCE':
	// 	// return transformFrameNode(node, options);
	// 	case 'GROUP':
	// 	// return transformGroupNode(node);
	// 	case 'TEXT':
	// 	// return transformTextNode(node, options);
	// 	case 'RECTANGLE':
	// 	// return transformRectangleNode(node, options);
	// 	case 'ELLIPSE':
	// 	// return transformEllipseNode(node, options);
	// 	case 'POLYGON':
	// 	// return transformPolygonNode(node, options);
	// 	case 'STAR':
	// 	// return transformStarNode(node, options);
	// 	case 'LINE':
	// 	case 'VECTOR':
	// 	case 'BOOLEAN_OPERATION':
	// 	// return transformToSVGNode(node, options);
	// 	default:
	// 		throw new UnsupportedFigmaNodeException(node);
	// }

	return null as any;
}
