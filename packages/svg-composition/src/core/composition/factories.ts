import { SVGRenderer, type TSVGRendererOptions } from '../render';
import { Composition, type TCompositionConfig } from './Composition';

export function createSVGComposition(config: TSVGCompositionConfig) {
	const { renderer: rendererOptions = {}, ...compositionConfig } = config;
	const composition = new Composition(compositionConfig);
	composition.registerRenderer(new SVGRenderer(composition, rendererOptions));
	return composition;
}

type TSVGCompositionConfig = {
	renderer?: TSVGRendererOptions;
} & Omit<TCompositionConfig, 'renderer'>;
