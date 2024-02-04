import type { COMP } from '@dyn/dtif';

export function resetNodeBundleTransform(node: COMP.NodeBundle): COMP.NodeBundle {
	node.relativeTransform = [1, 0, 0, 0, 1, 0, 0, 0, 1];
	return node;
}
