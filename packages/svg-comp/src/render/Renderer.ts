import type { SvgElementChanges } from '@/rust/dyn-svg-comp-api/bindings';

import type { Composition } from '../Composition';

export abstract class Renderer {
	private readonly _isCallbackBased: boolean;
	private readonly _comp: () => Composition; // TODO: Bad practice?

	constructor(composition: Composition, isCallbackBased: boolean) {
		this._comp = () => composition;
		this._isCallbackBased = isCallbackBased;

		composition.watchOutputEvent('ElementChanges', (event) => {
			this.applyElementChanges(event.changes);
		});
	}

	protected get composition(): Composition {
		return this._comp();
	}

	public get isCallbackBased(): boolean {
		return this._isCallbackBased;
	}

	public abstract applyElementChanges(elementChanges: SvgElementChanges): void;

	public abstract clear(): void;
}
