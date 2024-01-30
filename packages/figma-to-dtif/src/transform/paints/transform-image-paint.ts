import type { COMP } from '@dyn/dtif';

import { ExportImagePaintException } from '../../exceptions';
import type { TExportImageConfig } from '../../types';
import {
	exportFigmaImageData,
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
	const imageHash = paint.imageHash;
	if (imageHash == null) {
		throw new ExportImagePaintException(nodeIds, `No valid image hash found!`);
	}

	const { size, content: binary } = await exportFigmaImageData(imageHash, nodeIds);

	let content: COMP.ImageContentMixin['content'];
	if (config.export.mode === 'External') {
		const response = await config.export.uploadData(binary, { key: imageHash });
		content = {
			type: 'Url',
			url: response.url,
			contentType: 'PNG'
		};
	} else {
		content = {
			type: 'Binary',
			content: Array.from(binary),
			contentType: 'PNG'
		};
	}

	return {
		width: size.width,
		height: size.height,
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
