import type { bindings, TComposition, TFontWithContent } from './dtif';

export async function rustify(dtif: TComposition): Promise<bindings.DTIFComposition> {
	const finalDTIF: bindings.DTIFComposition = {
		version: dtif.version,
		name: dtif.name,
		height: dtif.height,
		width: dtif.width,
		rootNodeId: dtif.rootNodeId,
		nodes: dtif.nodes,
		paints: dtif.paints,
		fonts: await transformFonts(dtif.fonts),
		changes: dtif.changes
	};

	return finalDTIF;
}

async function transformFonts(
	fonts: Record<string, TFontWithContent>
): Promise<Record<string, bindings.FontWithContent>> {
	const transformedFonts: Record<string, bindings.FontWithContent> = {};

	// Check if content is a string (URL), then load the font, else use the existing number array
	for (const [key, font] of Object.entries(fonts)) {
		const content = typeof font.content === 'string' ? await loadFont(font.content) : font.content;
		transformedFonts[key] = { ...font, content };
	}

	return transformedFonts;
}

async function loadFont(url: string): Promise<number[]> {
	try {
		const response = await fetch(url);

		// Check if the response is okay
		if (!response.ok) {
			throw new Error(`HTTP error! status: ${response.status}`);
		}

		const arrayBuffer = await response.arrayBuffer();
		return Array.from(new Uint8Array(arrayBuffer));
	} catch (e) {
		console.error('Failed to load font:', e);
		return [];
	}
}
