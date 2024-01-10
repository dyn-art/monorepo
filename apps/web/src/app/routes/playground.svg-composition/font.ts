import { Font, FontMetadata } from '@dyn/svg-composition';

export const INTER_REGULAR: TFont = {
	id: 123,
	metadata: {
		name: 'Inter Regular',
		family: 'Inter',
		style: 'Normal',
		weight: 400
	},
	url: 'https://fonts.gstatic.com/s/inter/v13/UcCO3FwrK3iLTeHuS_fvQtMwCp50KnMw2boKoduKmMEVuLyfMZhrib2Bg-4.ttf'
};

export const ABEEZEE_ITALIC: TFont = {
	id: 124,
	metadata: {
		name: 'ABeeZee Italic',
		family: 'ABeeZee',
		style: 'Italic',
		weight: 400
	},
	url: 'http://fonts.gstatic.com/s/abeezee/v22/esDT31xSG-6AGleN2tCklZUCGpG-GQ.ttf'
};

export async function loadFontWithContent(font: TFont): Promise<Font> {
	const response = await fetch(font.url);
	const arrayBuffer = await response.arrayBuffer();
	return {
		metadata: font.metadata,
		content: {
			type: 'Binary',
			content: Array.from(new Uint8Array(arrayBuffer))
		}
	};
}

export function loadFontWithUrl(font: TFont): Font {
	return {
		metadata: font.metadata,
		content: {
			type: 'Url',
			url: font.url
		}
	};
}

export async function loadFonts(fonts: TFont[], inline = true): Promise<Record<string, Font>> {
	const loadedFonts: Record<string, Font> = {};
	await Promise.all(
		fonts.map(async (font) => {
			const loadedFont = inline ? await loadFontWithContent(font) : loadFontWithUrl(font);
			loadedFonts[font.id] = loadedFont;
		})
	);
	return loadedFonts;
}

type TFont = {
	metadata: FontMetadata;
	url: string;
	id: number;
};
