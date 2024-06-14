import type { CNV } from '../cnv';

/**
 * Prepares a DTIF canvas by resolving the URLs of assets.
 * This is necessary because the Rust part of the project cannot yet resolve URLs
 * due to immature WebAssembly (WASM) bindings.
 */
export async function prepareDtif(dtif: CNV.DtifCanvas): Promise<CNV.DtifCanvas> {
	return {
		...dtif,
		assets: await resolveAssets(dtif.assets ?? [])
	};
}

async function resolveAssets(assets: CNV.Asset[]): Promise<CNV.Asset[]> {
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
