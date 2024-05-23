import type { COMP } from '../comp';

/**
 * Prepares a DTIF composition by resolving the URLs of assets.
 * This is necessary because the Rust part of the project cannot yet resolve URLs
 * due to immature WebAssembly (WASM) bindings.
 */
export async function prepareDtifComposition(
	dtif: COMP.DtifComposition
): Promise<COMP.DtifComposition> {
	return {
		...dtif,
		assets: await resolveAssets(dtif.assets ?? [])
	};
}

async function resolveAssets(assets: COMP.Asset[]): Promise<COMP.Asset[]> {
	for (const asset of assets) {
		if (asset.content.type === 'Url') {
			const content = await loadContent(asset.content.url);
			asset.content = {
				type: 'Binary',
				content
			};
		}
	}
	return assets;
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
