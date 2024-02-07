import type { COMP } from '@dyn/dtif';

import { ExportFontException } from '../../exceptions';
import type { TToTransformFont } from '../../FigmaNodeTreeProcessor';
import type { TContentType, TExportConfig } from '../../types';
import { handleExport } from '../../utils';

export async function transformFont(
	toTransformFont: TToTransformFont,
	config: TTransformFontConfig
): Promise<COMP.FontContent> {
	const { fontMetadata, nodeIds } = toTransformFont;
	const { export: exportConfig, resolveFontContent } = config;

	// Resolve font
	let fontContent;
	try {
		fontContent = await resolveFontContent(fontMetadata);
		if (fontContent == null) {
			throw new ExportFontException(fontMetadata, nodeIds, 'No font found!');
		}
	} catch (error) {
		throw new ExportFontException(fontMetadata, nodeIds, error);
	}

	// Handle Url
	if (fontContent.type === 'Url') {
		return fontContent;
	}

	// Handle Binary
	return handleExport(fontContent.content, {
		export: exportConfig,
		contentType: fontContent.contentType
	});
}

export interface TTransformFontConfig {
	export: TExportConfig;
	resolveFontContent: TResolveFontContent;
}

export type TResolveFontContent = (fontMetadata: COMP.FontMetadata) => Promise<TFontContent>;

type TFontContent =
	| { type: 'Binary'; content: Uint8Array; contentType: TContentType }
	| { type: 'Url'; url: string; contentType: TContentType }
	| null;
