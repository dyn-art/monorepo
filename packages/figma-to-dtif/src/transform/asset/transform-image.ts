import type { CNV } from '@dyn/cnv-dtif';

import { ExportImageAssetException } from '../../exceptions';
import type { TToTransformImageAsset } from '../../FigmaNodeTreeProcessor';
import type { TExportImageConfig } from '../../types';
import { exportFigmaImageData, handleExport } from '../../utils';

export async function transformImageAsset(
	asset: TToTransformImageAsset,
	nodeIds: SceneNode['id'][],
	config: TTransformImageAssetConfig
): Promise<CNV.AssetWithId> {
	const { export: exportConfig } = config;
	const { hash: imageHash } = asset;

	// Resolve image
	if (imageHash == null) {
		throw new ExportImageAssetException(nodeIds, `No valid image hash found!`);
	}
	const image = await exportFigmaImageData(imageHash, nodeIds);

	// Upload image
	const content = await handleExport(image.content, {
		export: exportConfig,
		key: imageHash
	});

	return {
		contentType: { type: 'Unknown' },
		content
	};
}

export interface TTransformImageAssetConfig {
	export: TExportImageConfig;
}
