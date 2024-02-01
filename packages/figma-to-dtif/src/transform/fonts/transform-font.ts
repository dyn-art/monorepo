import type { COMP } from '@dyn/dtif';

import { ExportFontException } from '../../exceptions';
import type { TToTransformFont } from '../../FigmaNodeTreeProcessor';
import type { TContentType, TExportConfig } from '../../types';
import { uploadStaticData } from '../../utils';

export async function transformFont(
	toTransformFont: TToTransformFont,
	config: TTransformFontConfig
): Promise<COMP.Font> {
	const { fontMetadata, nodeIds } = toTransformFont;
	const { export: exportConfig, resolveFontContent } = config;

	// Resolve font
	let font;
	try {
		font = await resolveFontContent(toTransformFont.fontMetadata);
		if (font == null) {
			throw new ExportFontException(fontMetadata, nodeIds, 'No font found!');
		}
	} catch (error) {
		throw new ExportFontException(fontMetadata, nodeIds, error);
	}

	// Upload font
	const content = await uploadStaticData(font.content, {
		export: exportConfig,
		contentType: font.contentType
	});

	return {
		metadata: fontMetadata,
		content
	};
}

export interface TTransformFontConfig {
	export: TExportConfig;
	resolveFontContent: TResolveFontContent;
}

export type TResolveFontContent = (
	fontMetadata: COMP.FontMetadata
) => Promise<{ content: Uint8Array; contentType: TContentType } | null>;
