import { CanvasException } from './CanvasException';

export class FailedToResolveRootNodeException extends CanvasException {
	constructor() {
		super('Failed to resolve root node!');
	}
}
