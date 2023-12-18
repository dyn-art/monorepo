export function isFigmaTextNode(node: any): node is TextNode {
	return node?.type === 'TEXT';
}

export function isRemovedNode(node: any): node is RemovedNode {
	return typeof node?.removed === 'boolean' && node.removed;
}
