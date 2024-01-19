import type { COMP } from './comp';

export async function rustify(dtif: COMP.DTIFComposition): Promise<COMP.DTIFComposition> {
	const finalDTIF: COMP.DTIFComposition = {
		version: dtif.version,
		name: dtif.name,
		height: dtif.height,
		width: dtif.width,
		rootNodeId: dtif.rootNodeId,
		nodes: dtif.nodes,
		paints: dtif.paints,
		fonts: dtif.fonts != null ? await resolveFonts(dtif.fonts) : null,
		changes: dtif.changes
	};

	return finalDTIF;
}

async function resolveFonts(fonts: Record<string, COMP.Font>): Promise<Record<string, COMP.Font>> {
	const transformedFonts: Record<string, COMP.Font> = {};

	// Check if content is a string (URL), then load the font, else use the existing number array
	for (const [key, font] of Object.entries(fonts)) {
		if (font.content.type === 'Url') {
			const content = await loadFont(font.content.url);
			transformedFonts[key] = {
				...font,
				content: {
					type: 'Binary',
					content
				}
			};
		}
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
