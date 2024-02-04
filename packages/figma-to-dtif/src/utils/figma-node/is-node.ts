import type { TFigmaNodeWithChildren, TFigmaShapeNode } from '../../types';

// https://www.figma.com/plugin-docs/api/nodes/
export const SCENE_NODE_TYPES: NodeType[] = [
	'BOOLEAN_OPERATION',
	'CODE_BLOCK',
	'COMPONENT',
	'COMPONENT_SET',
	'CONNECTOR',
	'ELLIPSE',
	'EMBED',
	'FRAME',
	'GROUP',
	'INSTANCE',
	'LINE',
	'LINK_UNFURL',
	'MEDIA',
	'POLYGON',
	'RECTANGLE',
	'SHAPE_WITH_TEXT',
	'SLICE',
	'STAMP',
	'STAR',
	'STICKY',
	'TABLE',
	'TEXT',
	'VECTOR',
	'WIDGET'
];

export function isFigmaSceneNode(node: unknown): node is SceneNode {
	if (typeof node === 'object' && node !== null && 'type' in node && node.type != null) {
		const typedNode = node as { type: NodeType };
		return SCENE_NODE_TYPES.includes(typedNode.type);
	}
	return false;
}

export function isFigmaFrameNode(node: unknown): node is FrameNode {
	return isFigmaSceneNode(node) && node.type === 'FRAME';
}

export function isFigmaGroupNode(node: unknown): node is GroupNode {
	return isFigmaSceneNode(node) && node.type === 'GROUP';
}

export function isFigmaRectangleNode(node: unknown): node is RectangleNode {
	return isFigmaSceneNode(node) && node.type === 'RECTANGLE';
}

export function isFigmaLineNode(node: unknown): node is LineNode {
	return isFigmaSceneNode(node) && node.type === 'LINE';
}

export function isFigmaEllipseNode(node: unknown): node is EllipseNode {
	return isFigmaSceneNode(node) && node.type === 'ELLIPSE';
}

export function isFigmaPolygonNode(node: unknown): node is PolygonNode {
	return isFigmaSceneNode(node) && node.type === 'POLYGON';
}

export function isFigmaStarNode(node: unknown): node is StarNode {
	return isFigmaSceneNode(node) && node.type === 'STAR';
}

export function isFigmaVectorNode(node: unknown): node is VectorNode {
	return isFigmaSceneNode(node) && node.type === 'VECTOR';
}

export function isFigmaTextNode(node: unknown): node is TextNode {
	return typeof node === 'object' && node != null && 'type' in node && node.type === 'TEXT';
}

export function isFigmaBooleanOperationNode(node: unknown): node is BooleanOperationNode {
	return isFigmaSceneNode(node) && node.type === 'BOOLEAN_OPERATION';
}

export function isFigmaInstanceNode(node: unknown): node is InstanceNode {
	return isFigmaSceneNode(node) && node.type === 'INSTANCE';
}

export function isFigmaComponentNode(node: unknown): node is ComponentNode {
	return isFigmaSceneNode(node) && node.type === 'COMPONENT';
}

export function isFigmaRemovedNode(node: unknown): node is RemovedNode {
	return isFigmaSceneNode(node) && node.removed;
}

export function isFigmaNodeWithChildren(node: unknown): node is TFigmaNodeWithChildren {
	return (
		isFigmaFrameNode(node) ||
		isFigmaComponentNode(node) ||
		isFigmaInstanceNode(node) ||
		isFigmaGroupNode(node)
	);
}

export function isFigmaShapeNode(node: unknown): node is TFigmaShapeNode {
	return (
		isFigmaRectangleNode(node) ||
		isFigmaEllipseNode(node) ||
		isFigmaPolygonNode(node) ||
		isFigmaStarNode(node) ||
		isFigmaVectorNode(node)
	);
}
