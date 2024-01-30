import type { COMP } from './comp';

export async function rustify(dtif: COMP.DTIFComposition): Promise<COMP.DTIFComposition> {
	const finalDTIF: COMP.DTIFComposition = {
		version: dtif.version,
		name: dtif.name,
		height: dtif.height,
		width: dtif.width,
		rootNodeId: dtif.rootNodeId,
		nodes: dtif.nodes,
		paints: await resolvePaints(dtif.paints),
		fonts: dtif.fonts != null ? await resolveFonts(dtif.fonts) : null,
		changes: dtif.changes,
		viewBox: {
			width: dtif.width,
			height: dtif.height,
			minX: 0,
			minY: 0
		}
	};

	return finalDTIF;
}

async function resolvePaints(
	paints: Record<string, COMP.PaintBundle>
): Promise<Record<string, COMP.PaintBundle>> {
	for (const paint of Object.values(paints)) {
		if (paint.type === 'Image' && paint.imageContent.content.type === 'Url') {
			const content = await loadContent(paint.imageContent.content.url);
			paint.imageContent = {
				...paint.imageContent,
				content: {
					type: 'Binary',
					content,
					contentType: paint.imageContent.content.contentType
				}
			};
		}
	}
	return paints;
}

async function resolveFonts(fonts: Record<string, COMP.Font>): Promise<Record<string, COMP.Font>> {
	// Check if content is a string (URL), then load the font, else use the existing number array
	for (const font of Object.values(fonts)) {
		if (font.content.type === 'Url') {
			const content = await loadContent(font.content.url);
			font.content = {
				type: 'Binary',
				content
			};
		}
	}
	return fonts;
}

async function loadContent(url: string): Promise<number[]> {
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
