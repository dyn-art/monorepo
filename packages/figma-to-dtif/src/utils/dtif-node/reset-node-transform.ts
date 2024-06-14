import type { ARB } from '@dyn/arb-dtif';

export function resetDtifNodeTransform(node: ARB.Node): ARB.Node {
	node.translation = [0, 0];
	node.rotationDeg = 0;
	return node;
}
