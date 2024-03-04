import { shortId } from '@dyn/utils';
import { SvgCompHandle } from '@/rust/dyn-svg-comp-api';
import type {
	CompDtif,
	CompositionChangeOutputEvent,
	Entity,
	SelectionChangeOutputEvent,
	SvgCompInputEvent,
	SvgCompOutputEvent,
	SvgElementChangesOutputEvent,
	WatchedEntityChangesOutputEvent
} from '@/rust/dyn-svg-comp-api/bindings';

import type { Renderer } from './render';

export class Composition {
	private readonly _svgCompHandle: SvgCompHandle;
	private _renderer: Renderer | null = null;

	private _inputEventQueue: SvgCompInputEvent[] = [];

	private _watchedOutputEventCallbackMap: TWatchedOutputEventsCallbackMap = {};

	constructor(config: TCompositionConfig) {
		const { dtif, interactive = false } = config;
		this._svgCompHandle = SvgCompHandle.create(dtif, interactive);
	}

	// =========================================================================
	// Getter & Setter
	// =========================================================================

	public get renderer(): Renderer | null {
		return this._renderer;
	}

	public set renderer(value: Renderer) {
		this._renderer = value;
	}

	// =========================================================================
	// Lifecycle
	// =========================================================================

	public update(): void {
		const outputEvents: SvgCompOutputEvent[] = this._svgCompHandle.update(
			this._inputEventQueue.splice(0, this._inputEventQueue.length)
		);

		// Handle output events
		for (const event of outputEvents) {
			this.handleOutputEvent(event.type, event);
		}
	}

	public unmount(): void {
		this._svgCompHandle.free();
		this._renderer?.clear();
		this._renderer = null;
		this._watchedOutputEventCallbackMap = {};
		this._inputEventQueue = [];
	}

	// =========================================================================
	// Events
	// =========================================================================

	private handleOutputEvent<GEventType extends keyof TOutputEventTypeMap>(
		eventType: GEventType,
		event: TOutputEventTypeMap[GEventType]
	): void {
		const callbacks = this._watchedOutputEventCallbackMap[eventType];
		if (!Array.isArray(callbacks)) {
			return;
		}

		callbacks.forEach((entry) => {
			entry.callback(event);
		});
	}

	public watchOutputEvent<GEventType extends keyof TOutputEventTypeMap>(
		eventType: GEventType,
		callback: TWatchedOutputEventCallback<GEventType>
	): () => void {
		const id = shortId();
		const entry: TWatchedOutputEventCallbackEntry<GEventType> = { id, callback };

		const callbacks = this._watchedOutputEventCallbackMap[eventType];
		if (Array.isArray(callbacks)) {
			callbacks.push(entry);
		} else {
			// @ts-expect-error -- Entry is of type GEventType
			this._watchedOutputEventCallbackMap[eventType] = [entry];
		}

		// Return an unregister function
		return () => {
			this.unregisterOutputEventCallback(eventType, id);
		};
	}

	private unregisterOutputEventCallback<GEventType extends keyof TOutputEventTypeMap>(
		eventType: GEventType,
		id: string
	): void {
		const callbacks = this._watchedOutputEventCallbackMap[eventType];
		if (!Array.isArray(callbacks)) {
			return;
		}

		// @ts-expect-error -- Array has only elements of type GEventType
		this._watchedOutputEventCallbackMap[eventType] = callbacks.filter((entry) => entry.id !== id);
	}

	public emitInputEvent(event: SvgCompInputEvent, debounce = true): void {
		this._inputEventQueue.push(event);

		// TODO: debounce
		if (this.renderer?.isCallbackBased) {
			// TODO
		}
	}

	public emitInputEvents(events: SvgCompInputEvent[], debounce = true): void {
		for (const event of events) {
			this.emitInputEvent(event, debounce);
		}
	}

	// =========================================================================
	// Other
	// =========================================================================

	public logEntityComponentsRaw(entity: Entity): void {
		this._svgCompHandle.logEntityComponentsRaw(entity);
	}

	public toString(): string | null {
		return this._svgCompHandle.toString() ?? null;
	}
}

export interface TCompositionConfig {
	dtif: CompDtif;
	interactive?: boolean;
}

interface TOutputEventTypeMap {
	SvgElementChanges: SvgElementChangesOutputEvent;
	WatchedEntityChanges: WatchedEntityChangesOutputEvent;
	SelectionChange: SelectionChangeOutputEvent;
	CompositionChange: CompositionChangeOutputEvent;
}

type TWatchedOutputEventCallback<GEventType extends keyof TOutputEventTypeMap> = (
	value: TOutputEventTypeMap[GEventType]
) => void;

interface TWatchedOutputEventCallbackEntry<GEventType extends keyof TOutputEventTypeMap> {
	id: string;
	callback: TWatchedOutputEventCallback<GEventType>;
}

type TWatchedOutputEventsCallbackMap = {
	[K in SvgCompOutputEvent['type']]?: TWatchedOutputEventCallbackEntry<K>[];
};
