import type { COMP } from '@dyn/comp-dtif';
import { ContinuousId } from '@dyn/utils';

import type { Transformer } from '../../Transformer';
import type { TFigmaFormat } from '../../types';
import {
	exportFigmaNode,
	mapFigmaFormatToDtifContentType,
	mapFigmaTransformToRotation,
	mapFigmaTransformToTranslation
} from '../../utils';

export async function transformNodeToImage(
	node: SceneNode,
	cx: Transformer,
	config: TTransformNodeToImageConfig
): Promise<{ type: 'Rectangle' } & COMP.RectangleNode> {
	const { format } = config;

	// Export node as image
	const assetId = ContinuousId.nextId();
	const paintId = ContinuousId.nextId();
	const binary = await exportFigmaNode(node, { format });
	cx.insertAsset(assetId, {
		content: { type: 'Binary', content: Array.from(binary) },
		contentType: mapFigmaFormatToDtifContentType(format, node.width, node.height)
	});
	cx.insertPaint(paintId, {
		type: 'Image',
		assetId: assetId.toString(),
		scaleMode: { type: 'Fill' }
	});

	return {
		type: 'Rectangle',
		visible: node.visible,
		size: [node.width, node.height],
		translation: mapFigmaTransformToTranslation(node.relativeTransform),
		rotationDeg: mapFigmaTransformToRotation(node.relativeTransform),
		styles: [{ type: 'Fill', paintId: paintId.toString() }]
	};
}

interface TTransformNodeToImageConfig {
	format: TFigmaFormat;
}
