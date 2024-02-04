import { mat3, vec3 } from '@dyn/dtif';
import { shortId } from '@dyn/utils';
import { JsCompositionHandle } from '@/rust/dyn_svg_composition_api';
import type {
	AnyInputEvent,
	CompositionChange,
	CompositionChangeEvent,
	CoreInputEvent,
	CursorChangeEvent,
	CursorForFrontend,
	DTIFComposition,
	ElementChangeEvent,
	Entity,
	InteractionInputEvent,
	InteractionModeChangeEvent,
	InteractionModeForFrontend,
	MixinChange,
	NodeBundle,
	OutputEvent,
	PaintBundle,
	SelectionChangeEvent,
	TrackableMixinType,
	TrackUpdateEvent,
	ViewBox
} from '@/rust/dyn_svg_composition_api/bindings';

import type { TRustEnumKeyArray } from '../../wasm';
import type { Render } from '../render';
import { groupByType } from '../utils';

export class Composition {
	public readonly id: string;
	private readonly _compositionHandle: JsCompositionHandle;

	private _renderer: Render[] = [];

	private _width: number;
	private _height: number;
	private _viewBox: ViewBox;
	private readonly _isCallbackBased: boolean;

	private _eventQueue: AnyInputEvent[] = [];

	// https://www.zhenghao.io/posts/object-vs-map
	private _watchCompositionCallbacks = new Map<string, TWatchCompositionCallback>();
	private readonly _watchEntityCallbacks = new Map<Entity, Map<string, TWatchEntityCallback>>();
	private readonly _onSelectionChangeCallbacks = new Map<string, TOnSelectionChangeCallback>();
	private readonly _onInteractionModeChangeCallbacks = new Map<
		string,
		TOnInteractionModeChangeCallback
	>();
	private readonly _onCursorChangeCallbacks = new Map<string, TOnCursorChangeCallback>();

	// Interaction events debounce
	private debounceTimeout: number | null = null;
	private readonly debounceDelay: number = 100; // ms

	constructor(config: TCompositionConfig) {
		const { dtif, isCallbackBased = true } = config;
		this.id = shortId();
		this._compositionHandle = new JsCompositionHandle(dtif, (events: OutputEvent[]) => {
			this.onWasmEvents(events);
		});
		this._width = dtif.width;
		this._height = dtif.height;
		this._viewBox = dtif.viewBox ?? {
			width: dtif.width,
			height: dtif.height,
			minX: 0,
			minY: 0
		};
		this._isCallbackBased = isCallbackBased;
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

	public get viewBox(): Readonly<ViewBox> {
		return this._viewBox;
	}

	public get renderer(): Render[] {
		return this._renderer;
	}

	// =========================================================================
	// WASM
	// =========================================================================

	private onWasmEvents(events: OutputEvent[]): void {
		const groupedEvents = groupByType(events);
		for (const eventType of Object.keys(groupedEvents) as (keyof typeof groupedEvents)[]) {
			const groupedEvent = groupedEvents[eventType];
			if (groupedEvent != null) {
				switch (eventType) {
					case 'CompositionChange':
						this.handleCompositionUpdates(groupedEvent as CompositionChangeEvent[]);
						break;
					case 'ElementChange':
						this.handleElementUpdates(groupedEvent as ElementChangeEvent[]);
						break;
					case 'TrackUpdate':
						this.handleTrackUpdates(groupedEvent as TrackUpdateEvent[]);
						break;
					case 'SelectionChange':
						this.handleSelectionChanges(groupedEvent as SelectionChangeEvent[]);
						break;
					case 'InteractionModeChange':
						this.handleInteractionModeChanges(groupedEvent as InteractionModeChangeEvent[]);
						break;
					case 'CursorChange':
						this.handleCursorChanges(groupedEvent as CursorChangeEvent[]);
						break;
				}
			}
		}
	}

	// =========================================================================
	// Renderer
	// =========================================================================

	public registerRenderer(renderer: Render): void {
		this._renderer.push(renderer);
	}

	private handleElementUpdates(events: ElementChangeEvent[]): void {
		if (events.length > 0) {
			this._renderer.forEach((renderer) => {
				renderer.applyElementChanges(events);
			});
		}
	}

	private handleCompositionUpdates(events: CompositionChangeEvent[]): void {
		if (events.length > 0) {
			const { change } = events[events.length - 1] as unknown as CompositionChangeEvent;

			// Apply composition changes to each renderer
			this._renderer.forEach((renderer) => {
				renderer.applyCompositionChange(change);
			});

			this._viewBox = change.viewBox;
			this._width = change.width;
			this._height = change.height;

			// Call the watch composition callbacks with the aggregated changes
			this._watchCompositionCallbacks.forEach((callback) => {
				callback(change);
			});
		}
	}

	// =========================================================================
	// Watch Composition
	// =========================================================================

	public watchComposition(callback: TWatchCompositionCallback): () => void {
		const callbackId = shortId();
		this._watchCompositionCallbacks.set(callbackId, callback);
		return () => {
			this.unwatchComposition(callbackId);
		};
	}

	private unwatchComposition(callbackId: string): void {
		if (this._watchCompositionCallbacks.has(callbackId)) {
			this._watchCompositionCallbacks.delete(callbackId);
		}
	}

	// =========================================================================
	// Watch Entity
	// =========================================================================

	public watchEntity(
		entity: Entity,
		toTrackMixinKeys: TRustEnumKeyArray<TrackableMixinType>[],
		callback: TWatchEntityCallback,
		initalValue = true
	): (() => void) | false {
		// Enable tracking of entity in composition
		const intialChanges = this.trackEntity(
			entity,
			toTrackMixinKeys.map((key) => ({ type: key })),
			initalValue
		);

		// Apply intal changes if found
		if (Array.isArray(intialChanges)) {
			callback(entity, intialChanges);
		}

		// Register callback
		if (intialChanges) {
			const callbackId = shortId();
			let callbacks = this._watchEntityCallbacks.get(entity);
			if (!callbacks) {
				callbacks = new Map<string, TWatchEntityCallback>();
				this._watchEntityCallbacks.set(entity, callbacks);
			}
			callbacks.set(callbackId, callback);
			return () => {
				this.unwatchEntity(entity, callbackId);
			};
		}

		return false;
	}

	public unwatchEntity(entity: Entity, callbackId?: string): void {
		if (callbackId != null) {
			const callbacks = this._watchEntityCallbacks.get(entity);
			if (callbacks != null && callbacks.has(callbackId)) {
				// Unregister callback
				callbacks.delete(callbackId);

				// Disable tracking of entity in composition
				if (callbacks.size === 0) {
					this._watchEntityCallbacks.delete(entity);
					this.untrackEntity(entity);
				}
			}
		} else {
			this._watchEntityCallbacks.delete(entity);
		}
	}

	private trackEntity(
		entity: Entity,
		toTrackMixins: TrackableMixinType[],
		intialValue = true
	): MixinChange[] | boolean {
		return this._compositionHandle.trackEntity(entity, toTrackMixins, intialValue);
	}

	private untrackEntity(entity: Entity): boolean {
		return this._compositionHandle.untrackEntity(entity);
	}

	private handleTrackUpdates(events: TrackUpdateEvent[]): void {
		for (const event of events) {
			const callbacks = this._watchEntityCallbacks.get(event.id);
			if (callbacks != null) {
				callbacks.forEach((callback) => {
					callback(event.id, event.updates);
				});
			}
		}
	}

	// =========================================================================
	// Selection
	// =========================================================================

	public onSelectionChange(callback: TOnSelectionChangeCallback): () => void {
		const callbackId = shortId();
		this._onSelectionChangeCallbacks.set(callbackId, callback);
		return () => {
			this.unregisterOnSelectionChangeCallback(callbackId);
		};
	}

	private unregisterOnSelectionChangeCallback(callbackId: string): void {
		if (this._onSelectionChangeCallbacks.has(callbackId)) {
			this._onSelectionChangeCallbacks.delete(callbackId);
		}
	}

	private handleSelectionChanges(events: SelectionChangeEvent[]): void {
		if (events.length > 0) {
			this._onSelectionChangeCallbacks.forEach((callback) => {
				callback(events[events.length - 1]?.selected as unknown as Entity[]);
			});
		}
	}

	// =========================================================================
	// Interaction Mode
	// =========================================================================

	public onInteractionModeChange(callback: TOnInteractionModeChangeCallback): () => void {
		const callbackId = shortId();
		this._onInteractionModeChangeCallbacks.set(callbackId, callback);
		return () => {
			this.unregisterInteractionModeChangeCallback(callbackId);
		};
	}

	private unregisterInteractionModeChangeCallback(callbackId: string): void {
		if (this._onInteractionModeChangeCallbacks.has(callbackId)) {
			this._onInteractionModeChangeCallbacks.delete(callbackId);
		}
	}

	private handleInteractionModeChanges(events: InteractionModeChangeEvent[]): void {
		if (events.length > 0) {
			this._onInteractionModeChangeCallbacks.forEach((callback) => {
				callback(
					events[events.length - 1]?.interactionMode as unknown as InteractionModeForFrontend
				);
			});
		}
	}

	// =========================================================================
	// Cursor
	// =========================================================================

	public onCursorChange(callback: TOnCursorChangeCallback): () => void {
		const callbackId = shortId();
		this._onCursorChangeCallbacks.set(callbackId, callback);
		return () => {
			this.unregisterCursorChangeCallback(callbackId);
		};
	}

	private unregisterCursorChangeCallback(callbackId: string): void {
		if (this._onCursorChangeCallbacks.has(callbackId)) {
			this._onCursorChangeCallbacks.delete(callbackId);
		}
	}

	private handleCursorChanges(events: CursorChangeEvent[]): void {
		if (events.length > 0) {
			this._onCursorChangeCallbacks.forEach((callback) => {
				callback(events[events.length - 1]?.cursor as unknown as CursorForFrontend);
			});
		}
	}

	// =========================================================================
	// Lifecycle
	// =========================================================================

	public update(): void {
		this._compositionHandle.update(this._eventQueue);
		this._eventQueue = [];
	}

	// =========================================================================
	// Event
	// =========================================================================

	public emitCoreEvents(events: CoreInputEvent[]): void {
		this._eventQueue.push({ type: 'Core', events });
	}

	public emitInteractionEvents(events: InteractionInputEvent[], debounce = true): void {
		this._eventQueue.push({ type: 'Interaction', events });

		// Delay update call, resetting timer on new events within debounceDelay
		if (this._isCallbackBased) {
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

	// =========================================================================
	// Entity Creation
	// =========================================================================

	public spawnPaint(paint: PaintBundle): Entity {
		return this._compositionHandle.spawnPaint(paint);
	}

	public spawnRectangle(
		config: {
			x: number;
			y: number;
			width: number;
			height: number;
			color?: [number, number, number];
		},
		parentId?: Entity
	): Entity {
		const { x, y, width, height, color = [0, 0, 0] } = config;
		const paintId = this.spawnPaint({
			type: 'Solid',
			color
		});
		const bundle: NodeBundle = {
			type: 'Rectangle',
			dimension: {
				width,
				height
			},
			relativeTransform: mat3(vec3(1, 0, 0), vec3(0, 1, 0), vec3(x, y, 1)),
			rectangleCornerMixin: {
				bottomLeftRadius: 20
			},
			fill: {
				paintIds: [paintId]
			}
		};
		return this._compositionHandle.spawnNodeBundle(bundle, parentId);
	}

	// =========================================================================
	// Composition Interaction
	// =========================================================================

	public resize(width: number, height: number): void {
		this.emitCoreEvents([{ type: 'CompositionResized', width, height }]);
		this.update();
		this._width = width;
		this._height = height;
	}

	public updateViewBox(viewBox: ViewBox): void {
		this.emitCoreEvents([{ type: 'CompositionViewBoxChanged', viewBox }]);
		this.update();
		this._viewBox = viewBox;
	}

	// =========================================================================
	// Entity Interaction
	// =========================================================================

	public moveEntity(entity: Entity, dx: number, dy: number): void {
		this.emitCoreEvents([{ type: 'EntityMoved', entity, dx, dy }]);
	}

	public setEntityPosition(entity: Entity, x: number, y: number): void {
		this.emitCoreEvents([{ type: 'EntitySetPosition', entity, x, y }]);
	}

	// =========================================================================
	// Other
	// =========================================================================

	public clear(): void {
		this._renderer.forEach((renderer) => {
			renderer.clear();
		});
	}

	public unmount(): void {
		this.clear();
		this._compositionHandle.free();
		this._renderer = [];
		this._eventQueue = [];
	}

	public toString(): string | null {
		return this._compositionHandle.toString() ?? null;
	}
}

export interface TCompositionConfig {
	dtif: DTIFComposition;
	isCallbackBased?: boolean;
}

type TWatchCompositionCallback = (change: CompositionChange) => void;
type TWatchEntityCallback = (entity: Entity, changes: MixinChange[]) => void;
type TOnSelectionChangeCallback = (selected: Entity[]) => void;
type TOnInteractionModeChangeCallback = (interactionMode: InteractionModeForFrontend) => void;
type TOnCursorChangeCallback = (cursor: CursorForFrontend) => void;
