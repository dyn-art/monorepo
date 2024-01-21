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
): Promise<{ type: 'Image' } & COMP.ImagePaint> {
	const { content, size } = await resolveImage(paint, nodeIds, config);

	return {
		type: 'Image',
		content,
		scaleMode: resolveScaleMode(paint),
		width: size.width,
		height: size.height,
		blendMode: mapFigmaBlendModeToDTIF(paint.blendMode),
		opacity: paint.opacity ?? 1,
		isVisible: paint.visible ?? true
	};
}

async function resolveImage(
	paint: ImagePaint,
	nodeIds: SceneNode['id'][],
	config: TTransformImagePaintConfig
): Promise<{ content: COMP.ImagePaint['content']; size: { width: number; height: number } }> {
	const imageHash = paint.imageHash;
	if (imageHash == null) {
		throw new ExportImagePaintException(nodeIds, `No valid image hash found!`);
	}

	const { size, content: binary } = await exportFigmaImageData(imageHash, nodeIds);

	let content: COMP.ImagePaint['content'];
	if (config.export.mode === 'External') {
		const response = await config.export.uploadData(binary, { key: imageHash });
		content = {
			type: 'Url',
			url: response.url
		};
	} else {
		content = {
			type: 'Binary',
			content: Array.from(binary)
		};
	}

	return { content, size };
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
				type: 'Fill',
				rotation: paint.rotation ?? 0
			};
		case 'FIT':
			return {
				type: 'Fit',
				rotation: paint.rotation ?? 0
			};
			break;
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
