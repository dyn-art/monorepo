import type { CNV } from '@dyn/cnv-dtif';

import { UploadStaticDataException } from '../../exceptions';
import type { TContentType, TExportConfig } from '../../types';

export async function handleExport(
	binary: Uint8Array,
	config: TUploadStaticDataConfig
): Promise<CNV.AssetContent> {
	const { contentType, key, export: exportConfig, nodeIds = [] } = config;

	if (exportConfig.mode === 'External') {
		try {
			const response = await exportConfig.uploadData(binary, { key, contentType });
			return {
				type: 'Url',
				url: response.url
			};
		} catch (error) {
			throw new UploadStaticDataException(nodeIds, error);
		}
	}
	return {
		type: 'Binary',
		content: Array.from(binary)
	};
}

interface TUploadStaticDataConfig {
	contentType?: TContentType;
	export: TExportConfig;
	key?: string;
	nodeIds?: SceneNode['id'][];
}
