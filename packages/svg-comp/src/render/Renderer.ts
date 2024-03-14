import type {
	CompositionChangeOutputEvent,
	SvgElementChangesOutputEvent,
	Vec2
} from '@/rust/dyn-svg-comp-api/bindings';

import type { Composition } from '../Composition';

export abstract class Renderer {
	private readonly _isCallbackBased: boolean;
	private readonly _comp: () => Composition; // TODO: Bad practice?

	constructor(composition: Composition, isCallbackBased: boolean) {
		this._comp = () => composition;
		this._isCallbackBased = isCallbackBased;

		composition.watchOutputEvent('SvgElementChanges', (event) => {
			this.applyElementChanges(event);
		});
		composition.watchOutputEvent('CompositionChange', (event) => {
			this.applyCompositionChange(event);
		});
	}

	protected get composition(): Composition {
		return this._comp();
	}

	public get isCallbackBased(): boolean {
		return this._isCallbackBased;
	}

	public abstract applyElementChanges(event: SvgElementChangesOutputEvent): void;
	public abstract applyCompositionChange(event: CompositionChangeOutputEvent): void;
	public abstract clear(): void;
	public abstract clientWindowPointToCompPoint(clientPoint: Vec2): Vec2;

	public pointerEventToCompPoint(event: PointerEvent | { clientX: number; clientY: number }): Vec2 {
		return this.clientWindowPointToCompPoint([event.clientX, event.clientY]);
	}
}
