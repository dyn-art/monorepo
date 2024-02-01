import type { COMP } from '@dyn/dtif';

import { UploadStaticDataException } from '../../exceptions';
import type { TContentType, TExportConfig } from '../../types';

export async function uploadStaticData(
	binary: Uint8Array,
	config: TUploadStaticDataConfig
): Promise<COMP.ImageContentMixin['content']> {
	const { contentType, key, export: exportConfig, nodeIds = [] } = config;

	if (exportConfig.mode === 'External') {
		try {
			const response = await exportConfig.uploadData(binary, { key, contentType });
			return {
				type: 'Url',
				url: response.url,
				contentType: 'PNG'
			};
		} catch (error) {
			throw new UploadStaticDataException(nodeIds, error);
		}
	}
	return {
		type: 'Binary',
		content: Array.from(binary),
		contentType: 'PNG'
	};
}

interface TUploadStaticDataConfig {
	contentType?: TContentType;
	export: TExportConfig;
	key?: string;
	nodeIds?: SceneNode['id'][];
}
