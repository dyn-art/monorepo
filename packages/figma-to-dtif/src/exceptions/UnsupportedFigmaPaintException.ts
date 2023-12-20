import { NodeException } from './NodeException';

export class UnsupportedFigmaPaintException extends NodeException {
	constructor(paint: Paint, nodeId: SceneNode['id']) {
		super(
			`The Figma paint of the type '${paint.type}' in node '${nodeId}' is not supported yet!`,
			nodeId
		);
	}
}
