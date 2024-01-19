import type { COMP } from '@dyn/dtif';

export function isDTIFTextNode(node: any): node is COMP.TextNodeBundle {
	return node?.text != null;
}
