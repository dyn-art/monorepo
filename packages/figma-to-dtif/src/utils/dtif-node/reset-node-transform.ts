import type { TNode } from '@dyn/dtif';

export function resetDTIFNodeTransform(node: TNode): TNode {
	node.relativeTransform = [1, 0, 0, 0, 1, 0, 0, 0, 1];
	return node;
}
