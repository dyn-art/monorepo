import type { SvgElementChanges } from '@/rust/dyn-svg-comp-api/bindings';

import type { Composition } from '../Composition';

export abstract class Renderer {
	private _comp: () => Composition; // TODO: Bad practice?

	constructor(composition: Composition) {
		this._comp = () => composition;
	}

	protected get composition(): Composition {
		return this._comp();
	}

	public abstract applyElementChanges(elementChanges: SvgElementChanges): void;

	public abstract clear(): void;
}
