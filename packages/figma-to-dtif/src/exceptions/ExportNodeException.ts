import { NodeException } from './NodeException';

export class ExportNodeException extends NodeException {
	constructor(format: string, node: SceneNode) {
		super(`Failed to export node '${node.name}' as ${format}!`, node.id);
	}
}
