import type { CompositionChange, SvgElementChanges } from '@/rust/dyn-svg-comp-api/bindings';

import type { Composition } from '../Composition';

export abstract class Renderer {
	private readonly _isCallbackBased: boolean;
	private readonly _comp: () => Composition; // TODO: Bad practice?

	constructor(composition: Composition, isCallbackBased: boolean) {
		this._comp = () => composition;
		this._isCallbackBased = isCallbackBased;

		composition.watchOutputEvent('SvgElementChanges', (event) => {
			this.applyElementChanges(event.changes);
		});
		composition.watchOutputEvent('CompositionChange', (event) => {
			this.applyCompositionChange(event.change);
		});
	}

	protected get composition(): Composition {
		return this._comp();
	}

	public get isCallbackBased(): boolean {
		return this._isCallbackBased;
	}

	public abstract applyElementChanges(elementChanges: SvgElementChanges): void;
	public abstract applyCompositionChange(change: CompositionChange): void;
	public abstract clear(): void;
}
