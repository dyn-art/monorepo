import { Composition, type TCompositionConfig } from './Composition';

export function createComposition(config: TCompositionConfig): Composition {
	const composition = new Composition(config);
	composition.update();
	return composition;
}
