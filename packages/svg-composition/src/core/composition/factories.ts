import { SVGRender, type TSVGRendererOptions } from '../render';
import { Composition, type TCompositionConfig } from './Composition';

export function createSVGComposition(config: TSVGCompositionConfig): Composition {
	const { render: renderOptions = {}, ...compositionConfig } = config;
	const composition = new Composition(compositionConfig);
	composition.registerRenderer(new SVGRender(composition, renderOptions));
	return composition;
}

type TSVGCompositionConfig = {
	render?: TSVGRendererOptions;
} & Omit<TCompositionConfig, 'renderer'>;
