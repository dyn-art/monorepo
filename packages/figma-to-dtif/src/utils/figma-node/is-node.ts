export function isFigmaTextNode(node: any): node is TextNode {
	return node?.type === 'TEXT';
}
