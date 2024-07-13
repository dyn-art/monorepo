import { Artboard, type TArtboardConfig } from './Artboard';

export function createArtboard(config: TArtboardConfig): Artboard {
	const artboard = new Artboard(config);
	artboard.update();
	return artboard;
}
