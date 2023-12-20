import { CompositionException } from './CompositionException';

export class FailedToResolveRootNodeException extends CompositionException {
  constructor() {
    super('Failed to resolve root node!');
  }
}
