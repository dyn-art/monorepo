import { Canvas, type TCanvasConfig } from './Canvas';

export function createCanvas(config: TCanvasConfig): Canvas {
	const canvas = new Canvas(config);
	canvas.update();
	return canvas;
}
