import { SvgCompHandle } from '@/rust/dyn-svg-comp-api';
import type { SvgCompInputEvent, SvgCompOutputEvent } from '@/rust/dyn-svg-comp-api/bindings';

import type { Renderer } from './render';

export class Composition {
	private readonly _svgCompHandle: SvgCompHandle;
	private _renderer: Renderer | null = null;

	private _inputEventQueue: SvgCompInputEvent[] = [];

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
	// Events
	// =========================================================================

	private handleElementChangesEvent(event: SvgCompOutputEvent): void {
		if (this._renderer != null) {
			this._renderer.applyElementChanges(event.changes);
		}
	}

	public emitInputEvent(event: SvgCompInputEvent, debounce = true): void {
		this._inputEventQueue.push(event);

		// TODO: debounce
	}

	public emitInputEvents(events: SvgCompInputEvent[], debounce = true): void {
		for (const event of events) {
			this.emitInputEvent(event, debounce);
		}
	}
}
