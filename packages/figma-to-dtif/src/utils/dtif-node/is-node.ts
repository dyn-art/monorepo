import type { TTextNodeBundle } from '@dyn/dtif';

export function isDTIFTextNode(node: any): node is TTextNodeBundle {
	return node?.text != null;
}
