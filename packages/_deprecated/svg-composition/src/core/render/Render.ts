import type {
	CompositionChange,
	ElementChangeEvent
} from '@/rust/dyn_svg_composition_api/bindings';

import type { Composition } from '../composition';

export abstract class Render {
	private _composition: () => Composition; // TODO: Bad practice?

	constructor(composition: Composition) {
		this._composition = () => composition;
	}

	// =========================================================================
	// Getter & Setter
	// =========================================================================

	protected get composition(): Composition {
		return this._composition();
	}

	// =========================================================================
	// Abstract
	// =========================================================================

	public abstract applyElementChanges(events: ElementChangeEvent[]): void;

	public abstract applyCompositionChange(change: CompositionChange): void;

	public abstract clear(): void;
}
