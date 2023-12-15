import { NodeException } from './NodeException';

export class UnsupportedFigmaNodeException extends NodeException {
  constructor(node: SceneNode) {
    super(
      `The Figma node '${node.name}' of the type '${node.type}' is not supported yet!`,
      node
    );
  }
}
