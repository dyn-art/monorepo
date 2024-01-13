import type { RenderUpdateEvent } from '@/rust/dyn_svg_composition_api/bindings';

import type { Composition } from '../composition';

export abstract class Renderer {
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
	public abstract setSize(width: number, height: number): void;

	public abstract render(events: RenderUpdateEvent[]): void;

	public abstract clear(): void;
}
