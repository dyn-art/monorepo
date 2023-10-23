import { SVGRenderer, type TSVGRendererOptions } from '../render';
import { Canvas, type TCanvasConfig } from './Canvas';

export function createSVGCanvas(config: TSVGCanvasConfig) {
	const { renderer: rendererConfig = {}, ...canvasConfig } = config;
	const canvas = new Canvas(canvasConfig);
	const svgRenderer = new SVGRenderer(canvas, rendererConfig);
	canvas.registerRenderer(svgRenderer);
	return canvas;
}

type TSVGCanvasConfig = {
	renderer?: TSVGRendererOptions;
} & TCanvasConfig;
