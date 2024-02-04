import type { TExportExternalConfig } from '../../types';
import { exportFigmaNode } from './export-node';

export async function exportAndUploadFigmaNode(
	node: SceneNode,
	config: TExportAndUploadFigmaNodeConfig
): Promise<{ url: string }> {
	const { format, uploadData } = config;
	const data = await exportFigmaNode(node, { format });
	const response = await uploadData(data);
	return { url: response.url };
}

type TExportAndUploadFigmaNodeConfig = TExportExternalConfig & { format: 'PNG' | 'JPG' };
