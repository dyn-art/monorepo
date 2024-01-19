import type {
	CompositionChangeEvent,
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

	public abstract applyCompositionChanges(events: CompositionChangeEvent[]): void;

	public abstract clear(): void;
}
