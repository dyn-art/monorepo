import type { SvgElementChanges } from '@/rust/dyn-svg-comp-api/bindings';

import type { Composition } from '../Composition';

export abstract class Renderer {
	private _composition: () => Composition; // TODO: Bad practice?

	constructor(composition: Composition) {
		this._composition = () => composition;
	}

	protected get composition(): Composition {
		return this._composition();
	}

	public abstract applyElementChanges(elementChanges: SvgElementChanges): void;

	public abstract clear(): void;
}
