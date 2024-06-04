import { toArray } from '@ibg/utils';

export class NodeException extends Error {
	public readonly nodeIds: SceneNode['id'][];

	constructor(message: string, nodeIds: SceneNode['id'] | SceneNode['id'][]) {
		super(message);
		this.nodeIds = toArray(nodeIds);
	}
}
