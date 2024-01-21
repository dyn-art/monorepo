import type { COMP } from '@dyn/dtif';

import type { TToTransformFont } from '../../FigmaNodeTreeProcessor';
import type { TContentType, TExportConfig } from '../../types';

export async function transformFont(
	toTransformFont: TToTransformFont,
	config: TTransformFontConfig
): Promise<COMP.Font> {
	// TODO
	return null as any;
}

export interface TTransformFontConfig {
	export: TExportConfig;
	resolveFontContent: TResolveFontContent;
}

export type TResolveFontContent = (
	fontMetadata: COMP.FontMetadata
) => Promise<{ content: Uint8Array | null; contentType: TContentType }>;
