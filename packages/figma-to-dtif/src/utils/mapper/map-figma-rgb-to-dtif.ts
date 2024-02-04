export function mapFigmaRGBToDTIF(rgb: RGB): [number, number, number] {
	return [rgb.r, rgb.g, rgb.b].map((value) => Math.round(value * 255)) as [number, number, number];
}
