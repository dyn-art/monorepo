import { ArtboardException } from './ArtboardException';

export class FailedToResolveRootNodeException extends ArtboardException {
	constructor() {
		super('Failed to resolve root node!');
	}
}
