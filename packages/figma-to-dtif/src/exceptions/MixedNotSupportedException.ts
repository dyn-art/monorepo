import { NodeException } from './NodeException';

export class MixedNotSupportedException extends NodeException {
	constructor(propertyName: string, node: SceneNode) {
		super(
			`The property '${propertyName}' in node '${node.name}' contains mixed values, which are not supported in this context. Please ensure that the font name is consistent across the text node.`,
			node.id
		);
	}
}
