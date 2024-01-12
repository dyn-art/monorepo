export async function fetchFont(url: string): Promise<ArrayBuffer> {
	const fontResponse = await fetch(url);
	return await fontResponse.arrayBuffer();
}
