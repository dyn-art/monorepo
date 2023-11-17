import { SVGRenderer, type TSVGRendererOptions } from '../render';
import { Composition, type TCompositionConfig } from './Composition';

export function createSVGComposition(config: TSVGCompositionConfig) {
	const { renderer: rendererConfig = {}, ...compositionConfig } = config;
	const composition = new Composition({
		...compositionConfig,
		renderer: new SVGRenderer(rendererConfig)
	});
	return composition;
}

type TSVGCompositionConfig = {
	renderer?: TSVGRendererOptions;
} & Omit<TCompositionConfig, 'renderer'>;
