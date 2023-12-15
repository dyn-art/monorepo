import { NodeException } from './NodeException';

export class InvisibleNodeException extends NodeException {
  constructor(node: SceneNode) {
    super(
      `Node '${node.name}' couldn't be exported because it is invisible.`,
      node
    );
  }
}
