import type { DTIFComposition } from '@dyn/svg-composition/bindings';

export class Transformer {
	// Nodes
	private readonly _toTransformRootNode: FrameNode;

	constructor(node: FrameNode) {
		this._toTransformRootNode = node;
	}

	public async transform(): Promise<DTIFComposition> {
		// TODO
		return null as any;
	}
}
