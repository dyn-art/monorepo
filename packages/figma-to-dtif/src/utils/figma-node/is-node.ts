export function isTextNode(node: any): node is TextNode {
	return node?.type === 'TEXT';
}
