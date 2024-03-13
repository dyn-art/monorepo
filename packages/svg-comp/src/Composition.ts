import { shortId } from '@dyn/utils';
import { SvgCompHandle } from '@/rust/dyn-svg-comp-api';
import type {
	ComponentChange,
	CompositionChangeOutputEvent,
	DtifComposition,
	Entity,
	SelectionChangeOutputEvent,
	SvgCompInputEvent,
	SvgCompOutputEvent,
	SvgElementChangesOutputEvent,
	Viewport,
	WatchableComponentVariant,
	WatchedEntityChangesOutputEvent
} from '@/rust/dyn-svg-comp-api/bindings';

import type { Renderer } from './render';

export class Composition {
	private readonly _svgCompHandle: SvgCompHandle;
	private _renderer: Renderer | null = null;

	private _width: number;
	private _height: number;
	private _viewport: Viewport;

	private _inputEventQueue: SvgCompInputEvent[] = [];
	private _watchedOutputEventCallbackMap: TWatchedOutputEventsCallbackMap = {};

	// Interaction events debounce
	private debounceTimeout: number | null = null;
	private readonly debounceDelay: number = 100; // ms

	constructor(config: TCompositionConfig) {
		const { dtif, interactive = false } = config;
		this._svgCompHandle = SvgCompHandle.create(dtif, interactive);
		this.watchOutputEvent('CompositionChange', (event) => {
			this._width = event.size[0];
			this._height = event.size[1];
			this._viewport = event.viewport;
		});
	}

	// =========================================================================
	// Getter & Setter
	// =========================================================================

	public get width(): number {
		return this._width;
	}

	public get height(): number {
		return this._height;
	}

	public get viewport(): Readonly<Viewport> {
		return this.viewport;
	}

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

	public watchEntity(
		entity: Entity,
		toWatchComponents: WatchableComponentVariant[],
		callback: TWatchEntityCallback,
		initialValue = true
	): () => void {
		const intialChanges = this._svgCompHandle.watchEntity(entity, toWatchComponents, initialValue);

		// Apply intal changes if found
		if (initialValue && Array.isArray(intialChanges)) {
			callback(entity, intialChanges);
		}

		// Register callback
		const unregister = this.watchOutputEvent('WatchedEntityChanges', (event) => {
			if (event.entity === entity) {
				callback(entity, event.changes);
			}
		});

		return () => {
			unregister();
			this._svgCompHandle.unregisterEntityCallback(entity);
		};
	}

	public emitInputEvent(event: SvgCompInputEvent, debounce = true): void {
		this._inputEventQueue.push(event);

		// Delay update call, resetting timer on new events within debounceDelay
		if (event.type === 'Interaction' && this.renderer?.isCallbackBased) {
			if (this.debounceTimeout != null) {
				clearTimeout(this.debounceTimeout);
			}
			if (debounce) {
				this.debounceTimeout = setTimeout(() => {
					this.update();
				}, this.debounceDelay) as unknown as number;
			} else {
				this.update();
			}
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
	dtif: DtifComposition;
	interactive?: boolean;
}

export interface TOutputEventTypeMap {
	SvgElementChanges: SvgElementChangesOutputEvent;
	WatchedEntityChanges: WatchedEntityChangesOutputEvent;
	SelectionChange: SelectionChangeOutputEvent;
	CompositionChange: CompositionChangeOutputEvent;
}

export type TWatchedOutputEventCallback<GEventType extends keyof TOutputEventTypeMap> = (
	value: TOutputEventTypeMap[GEventType]
) => void;

interface TWatchedOutputEventCallbackEntry<GEventType extends keyof TOutputEventTypeMap> {
	id: string;
	callback: TWatchedOutputEventCallback<GEventType>;
}

type TWatchedOutputEventsCallbackMap = {
	[K in SvgCompOutputEvent['type']]?: TWatchedOutputEventCallbackEntry<K>[];
};

export type TWatchEntityCallback = (entity: Entity, change: ComponentChange[]) => void;
