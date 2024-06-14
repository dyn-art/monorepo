import type { CNV } from '@dyn/cnv-dtif';

export function resetDtifNodeTransform(node: CNV.Node): CNV.Node {
	node.translation = [0, 0];
	node.rotationDeg = 0;
	return node;
}
