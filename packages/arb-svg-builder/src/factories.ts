import { Artboard, type TArtboardConfig } from './Artboard';

export function createArtboard(config: TArtboardConfig): Artboard {
	const canvas = new Artboard(config);
	canvas.update();
	return canvas;
}
