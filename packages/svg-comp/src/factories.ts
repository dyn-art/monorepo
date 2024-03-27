import { Composition, type TCompositionConfig } from './Composition';
import { SvgRenderer, type TsvgRendererOptions } from './render';

export function createComposition(config: TCompositionConfig): Composition {
	const composition = new Composition(config);
	composition.update();
	return composition;
}

export function createSvgComposition(config: TSvgCompositionConfig): Composition {
	const { renderer: renderOptions = {}, ...compositionConfig } = config;
	const composition = new Composition(compositionConfig);
	composition.renderer = new SvgRenderer(composition, renderOptions);
	composition.update(); // Inital update to sync viewport, ..
	return composition;
}

type TSvgCompositionConfig = {
	renderer?: TsvgRendererOptions;
} & TCompositionConfig;
