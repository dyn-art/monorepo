import { SVGRenderer, type TSVGRendererOptions } from '../render';
import { Composition, type TCompositionConfig } from './Composition';

export function createSVGComposition(config: TSVGCompositionConfig) {
	const { renderer: rendererConfig = {}, ...compositionConfig } = config;
	const composition = new Composition(compositionConfig);
	const svgRenderer = new SVGRenderer(composition, rendererConfig);
	composition.registerRenderer(svgRenderer);
	return composition;
}

type TSVGCompositionConfig = {
	renderer?: TSVGRendererOptions;
} & TCompositionConfig;
