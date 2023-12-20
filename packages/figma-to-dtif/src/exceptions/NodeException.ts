export class NodeException extends Error {
	public readonly nodeId: SceneNode['id'];

	constructor(message: string, nodeId: SceneNode['id']) {
		super(message);
		this.nodeId = nodeId;
	}
}
