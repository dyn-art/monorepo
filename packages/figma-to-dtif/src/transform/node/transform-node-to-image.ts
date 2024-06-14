import { ContinuousId } from '@ibg/utils';
import type { CNV } from '@dyn/cnv-dtif';

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
): Promise<{ type: 'Rectangle' } & CNV.RectangleNode> {
	const { format } = config;

	// Export node as image
	const imageId = ContinuousId.nextId();
	const paintId = ContinuousId.nextId();
	const binary = await exportFigmaNode(node, { format });
	cx.insertAsset(imageId, {
		content: { type: 'Binary', content: Array.from(binary) },
		contentType: mapFigmaFormatToDtifContentType(format, node.width, node.height)
	});
	cx.insertPaint(paintId, {
		type: 'Image',
		imageId: { type: 'ReferenceId', referenceId: `a${imageId}` },
		scaleMode: { type: 'Fill' }
	});

	return {
		type: 'Rectangle',
		visible: node.visible,
		size: [node.width, node.height],
		translation: mapFigmaTransformToTranslation(node.relativeTransform),
		rotationDeg: mapFigmaTransformToRotation(node.relativeTransform),
		styles: [{ type: 'Fill', paintId: { type: 'ReferenceId', referenceId: `p${paintId}` } }]
	};
}

interface TTransformNodeToImageConfig {
	format: TFigmaFormat;
}
