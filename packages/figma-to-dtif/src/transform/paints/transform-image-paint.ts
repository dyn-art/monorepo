import type { COMP } from '@dyn/dtif';

import { ExportImagePaintException } from '../../exceptions';
import type { TExportImageConfig } from '../../types';
import {
	exportFigmaImageData,
	handleExport,
	mapFigmaBlendModeToDTIF,
	mapFigmaTransformToMat3
} from '../../utils';

export async function transformImagePaint(
	paint: ImagePaint,
	nodeIds: SceneNode['id'][],
	config: TTransformImagePaintConfig
): Promise<{ type: 'Image' } & COMP.ImagePaintBundle> {
	const imageContent = await resolveImage(paint, nodeIds, config);

	return {
		type: 'Image',
		compositionMixin: {
			isVisible: paint.visible ?? true
		},
		imageContent,
		scaleMode: resolveScaleMode(paint),
		blendMixin: {
			blendMode: mapFigmaBlendModeToDTIF(paint.blendMode),
			opacity: paint.opacity ?? 1
		}
	};
}

async function resolveImage(
	paint: ImagePaint,
	nodeIds: SceneNode['id'][],
	config: TTransformImagePaintConfig
): Promise<COMP.ImageContentMixin> {
	const { export: exportConfig } = config;

	// Resolve image
	const imageHash = paint.imageHash;
	if (imageHash == null) {
		throw new ExportImagePaintException(nodeIds, `No valid image hash found!`);
	}
	const image = await exportFigmaImageData(imageHash, nodeIds);

	// Upload image
	const content = await handleExport(image.content, {
		export: exportConfig,
		key: imageHash
	});

	return {
		width: image.size.width,
		height: image.size.height,
		content
	};
}

function resolveScaleMode(paint: ImagePaint): COMP.ImagePaint['scaleMode'] {
	switch (paint.scaleMode) {
		case 'CROP':
			return {
				type: 'Crop',
				transform: mapFigmaTransformToMat3(
					paint.imageTransform ?? [
						[0, 0, 1],
						[1, 0, 0]
					]
				)
			};
		case 'FILL':
			return {
				type: 'Fill'
			};
		case 'FIT':
			return {
				type: 'Fit'
			};
		case 'TILE':
			return {
				type: 'Tile',
				rotation: paint.rotation ?? 0,
				scalingFactor: paint.scalingFactor ?? 1
			};
	}
}

export interface TTransformImagePaintConfig {
	export: TExportImageConfig;
}
