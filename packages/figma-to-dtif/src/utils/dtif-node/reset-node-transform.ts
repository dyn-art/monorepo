import type { COMP } from '@dyn/dtif-comp';

export function resetDtifNodeTransform(node: COMP.Node): COMP.Node {
	node.translation = [0, 0];
	node.angle = 0;
	return node;
}
