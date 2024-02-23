import { SvgCompHandle } from '@/rust/dyn-svg-comp-api';
import type { CompInputEvent, SvgCompOutputEvent } from '@/rust/dyn-svg-comp-api/bindings';

import type { Renderer } from './render';

export class Composition {
	private readonly _svgCompHandle: SvgCompHandle;
	private _renderer: Renderer | null = null;

	private _inputEventQueue: CompInputEvent[] = [];

	constructor() {
		this._svgCompHandle = new SvgCompHandle();
	}

	// =========================================================================
	// Getter & Setter
	// =========================================================================

	// TODO

	// =========================================================================
	// Lifecycle
	// =========================================================================

	public update(): void {
		const outputEvents: SvgCompOutputEvent[] = this._svgCompHandle.update(
			this._inputEventQueue.splice(0, this._inputEventQueue.length)
		);

		// Handle output events
		for (const event of outputEvents) {
			switch (event.type) {
				case 'ElementChanges':
					this.handleElementChangesEvent(event);
					break;
			}
		}
	}

	// =========================================================================
	// Event Handling
	// =========================================================================

	private handleElementChangesEvent(event: SvgCompOutputEvent): void {
		if (this._renderer != null) {
			this._renderer.applyElementChanges(event.changes);
		}
	}
}
