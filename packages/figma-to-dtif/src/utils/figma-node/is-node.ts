import type { TFigmaNodeWithChildren, TFigmaShapeNode } from '../../types';

export function isFigmaFrameNode(node: any): node is FrameNode {
	return node?.type === 'FRAME';
}

export function isFigmaGroupNode(node: any): node is GroupNode {
	return node?.type === 'GROUP';
}

export function isFigmaRectangleNode(node: any): node is RectangleNode {
	return node?.type === 'RECTANGLE';
}

export function isFigmaLineNode(node: any): node is LineNode {
	return node?.type === 'LINE';
}

export function isFigmaEllipseNode(node: any): node is EllipseNode {
	return node?.type === 'ELLIPSE';
}

export function isFigmaPolygonNode(node: any): node is PolygonNode {
	return node?.type === 'POLYGON';
}

export function isFigmaStarNode(node: any): node is StarNode {
	return node?.type === 'STAR';
}

export function isFigmaVectorNode(node: any): node is VectorNode {
	return node?.type === 'VECTOR';
}

export function isFigmaTextNode(node: any): node is TextNode {
	return node?.type === 'TEXT';
}

export function isFigmaBooleanOperationNode(node: any): node is BooleanOperationNode {
	return node?.type === 'BOOLEAN_OPERATION';
}

export function isFigmaInstanceNode(node: any): node is InstanceNode {
	return node?.type === 'INSTANCE';
}

export function isFigmaComponentNode(node: any): node is ComponentNode {
	return node?.type === 'COMPONENT';
}

export function isFigmaRemovedNode(node: any): node is RemovedNode {
	return typeof node?.removed === 'boolean' && node.removed;
}

export function isFigmaNodeWithChildren(node: any): node is TFigmaNodeWithChildren {
	return (
		isFigmaFrameNode(node) ||
		isFigmaComponentNode(node) ||
		isFigmaInstanceNode(node) ||
		isFigmaGroupNode(node)
	);
}

export function isFigmaShapeNode(node: any): node is TFigmaShapeNode {
	return (
		isFigmaRectangleNode(node) ||
		isFigmaEllipseNode(node) ||
		isFigmaPolygonNode(node) ||
		isFigmaStarNode(node)
	);
}
