import type { TExportImageConfig } from '../../types';
import { exportFigmaNode } from './export-node';

export async function exportAndUploadFigmaNode(
	node: SceneNode,
	config: TExportAndUploadFigmaNodeConfig
) {
	const { export: exportConfig } = config;
	const data = await exportFigmaNode(node, { format: exportConfig.format });

	if (exportConfig.mode === 'External') {
		const response = exportConfig.uploadData(data);
		// TODO:
	}
}

interface TExportAndUploadFigmaNodeConfig {
	export: TExportImageConfig;
}
