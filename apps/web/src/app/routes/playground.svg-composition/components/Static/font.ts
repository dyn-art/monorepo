import { FontMetadata, FontWithContent } from '@dyn/svg-composition';

export const INTER_REGULAR: TFont = {
	hash: 123,
	metadata: {
		name: 'Inter Regular',
		family: 'Inter',
		style: 'Normal',
		weight: 400
	},
	url: 'https://fonts.gstatic.com/s/inter/v13/UcCO3FwrK3iLTeHuS_fvQtMwCp50KnMw2boKoduKmMEVuLyfMZhrib2Bg-4.ttf'
};

export const ABEEZEE_ITALIC: TFont = {
	hash: 124,
	metadata: {
		name: 'ABeeZee Italic',
		family: 'ABeeZee',
		style: 'Italic',
		weight: 400
	},
	url: 'http://fonts.gstatic.com/s/abeezee/v22/esDT31xSG-6AGleN2tCklZUCGpG-GQ.ttf'
};

export async function loadFont(font: TFont): Promise<FontWithContent> {
	const response = await fetch(font.url);
	const arrayBuffer = await response.arrayBuffer();
	return {
		metadata: font.metadata,
		content: Array.from(new Uint8Array(arrayBuffer)),
		hash: font.hash
	};
}

export async function loadFonts(fonts: TFont[]): Promise<FontWithContent[]> {
	return Promise.all(fonts.map(async (font) => await loadFont(font)));
}

type TFont = {
	metadata: FontMetadata;
	url: string;
	hash: number;
};