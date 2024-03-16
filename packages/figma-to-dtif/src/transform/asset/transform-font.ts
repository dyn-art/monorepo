import type { COMP } from '@dyn/dtif-comp';

import { ExportFontAssetException } from '../../exceptions';
import type { TToTransformFontAsset } from '../../FigmaNodeTreeProcessor';
import type { TContentType, TExportConfig } from '../../types';
import { handleExport } from '../../utils';

export async function transformFontAsset(
	asset: TToTransformFontAsset,
	nodeIds: SceneNode['id'][],
	config: TTransformFontAssetConfig
): Promise<COMP.Asset> {
	const { export: exportConfig, resolveFontContent } = config;
	const { metadata: fontMetadata } = asset;

	// Resolve font
	let fontContent;
	try {
		fontContent = await resolveFontContent(asset.metadata);
		if (fontContent == null) {
			throw new ExportFontAssetException(fontMetadata, nodeIds, 'No font found!');
		}
	} catch (error) {
		throw new ExportFontAssetException(fontMetadata, nodeIds, error);
	}

	// Handle Url
	if (fontContent.type === 'Url') {
		return {
			content: fontContent,
			contentType: { type: 'Ttf' }
		};
	}

	// Handle Binary
	const content = await handleExport(fontContent.content, {
		export: exportConfig,
		contentType: fontContent.contentType
	});
	return {
		content,
		contentType: { type: 'Ttf' }
	};
}

export interface TTransformFontAssetConfig {
	export: TExportConfig;
	resolveFontContent: TResolveFontContent;
}

export type TResolveFontContent = (fontMetadata: COMP.FontMetadata) => Promise<TFontContent>;

type TFontContent =
	| { type: 'Binary'; content: Uint8Array; contentType: TContentType }
	| { type: 'Url'; url: string; contentType: TContentType }
	| null;
