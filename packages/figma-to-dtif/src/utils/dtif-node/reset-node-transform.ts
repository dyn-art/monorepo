import type { COMP } from '@dyn/comp-dtif';

export function resetDtifNodeTransform(node: COMP.Node): COMP.Node {
	node.translation = [0, 0];
	node.angle = 0;
	return node;
}
