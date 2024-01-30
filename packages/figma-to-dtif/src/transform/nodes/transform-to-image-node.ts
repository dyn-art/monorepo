import type { COMP } from '@dyn/dtif';
import { ContinuousId } from '@dyn/utils';

import type { Transformer } from '../../Transformer';
import { exportFigmaNode, mapDTIFContentTypeToFigma, mapFigmaTransformToMat3 } from '../../utils';

export async function transformToImageNode(
	node: SceneNode,
	cx: Transformer,
	config: TTransformToImageNode
): Promise<{ type: 'Rectangle' } & COMP.RectangleNodeBundle> {
	const { contentType } = config;

	// Export node to image
	const imagePaintId = ContinuousId.nextId();
	const binary = await exportFigmaNode(node, { format: mapDTIFContentTypeToFigma(contentType) });
	cx.insertPaint(imagePaintId, {
		type: 'Image',
		imageContent: {
			width: node.width,
			height: node.height,
			content: { type: 'Binary', contentType, content: Array.from(binary) }
		}
	});

	return {
		type: 'Rectangle',
		node: {
			node_type: 'Rectangle'
		},
		name: node.name,
		compositionMixin: {
			isLocked: node.locked,
			isVisible: node.visible
		},
		dimension: {
			height: node.height,
			width: node.width
		},
		relativeTransform: mapFigmaTransformToMat3(node.relativeTransform),
		fill: { paintIds: [imagePaintId] }
	};
}

interface TTransformToImageNode {
	contentType: COMP.ContentType;
}
