import type { TextNodeBundle } from '@dyn/svg-composition/bindings';

export function isDTIFTextNode(node: any): node is TextNodeBundle {
	return node?.text != null;
}
