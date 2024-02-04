export function createFigmaPaintNode(
	paints: Paint[],
	config: TCreateFigmaPaintNodeConfig
): SceneNode {
	const { width, height, exportContainerNode } = config;

	// Create paint node
	const paintNode = figma.createRectangle();
	paintNode.resize(width, height);
	paintNode.fills = paints;

	// Append paint node node to container node for context
	try {
		exportContainerNode.appendChild(paintNode);
	} finally {
		paintNode.remove();
	}

	return paintNode;
}

export interface TCreateFigmaPaintNodeConfig {
	width: number;
	height: number;
	exportContainerNode: FrameNode;
}
