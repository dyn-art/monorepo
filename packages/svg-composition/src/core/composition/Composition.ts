import { shortId } from '@dyn/utils';
import { JsCompositionHandle } from '@/rust/dyn_composition_api';
import type {
	AnyInputEvent,
	CoreInputEvent,
	DTIFComposition,
	Entity,
	InteractionInputEvent,
	MixinChange,
	OutputEvent,
	Paint,
	RectangleNodeBundle,
	RenderUpdateEvent,
	SelectionChangeEvent,
	TrackableMixinType,
	TrackUpdateEvent
} from '@/rust/dyn_composition_api/bindings';

import type { TRustEnumKeyArray } from '../../wasm';
import { groupByType, mat3, vec3 } from '../helper';
import type { Renderer } from '../render';

export class Composition {
	private readonly _compositionHandle: JsCompositionHandle;

	private _renderer: Renderer[] = [];

	private _width: number;
	private _height: number;
	private readonly _isCallbackBased: boolean;

	private _eventQueue: AnyInputEvent[] = [];

	// https://www.zhenghao.io/posts/object-vs-map
	private readonly _watchEntityCallbacks = new Map<Entity, Map<string, TWatchEntityCallback>>();
	private readonly _onSelectionChangeCallbacks = new Map<string, TOnSelectionChangeCallback>();

	// Interaction events debounce
	private debounceTimeout: number | null = null;
	private readonly debounceDelay: number = 100; // ms

	constructor(config: TCompositionConfig) {
		const {
			width,
			height,
			dtif = {
				version: '0.0.1',
				name: 'Test',
				width,
				height,
				rootNodeId: 0,
				nodes: {
					0: {
						type: 'Frame',
						children: [],
						dimension: {
							width,
							height
						},
						relativeTransform: mat3(vec3(1, 0, 0), vec3(0, 1, 0), vec3(0, 0, 1)),
						fill: {
							paints: [10]
						}
					}
				},
				paints: {
					10: {
						type: 'Solid',
						blendMode: 'Normal',
						color: [0, 191, 255],
						isVisible: true,
						opacity: 1
					}
				}
			},
			isCallbackBased = true
		} = config;
		this._compositionHandle = new JsCompositionHandle(dtif, (events: OutputEvent[]) => {
			this.onWasmEvents(events);
		});
		this._width = width;
		this._height = height;
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

	// =========================================================================
	// WASM
	// =========================================================================

	private onWasmEvents(events: OutputEvent[]): void {
		const groupedEvents = groupByType(events);
		for (const eventType of Object.keys(groupedEvents) as (keyof typeof groupedEvents)[]) {
			const groupedEvent = groupedEvents[eventType];
			if (groupedEvent != null) {
				switch (eventType) {
					case 'RenderUpdate':
						this.handleRenderUpdates(groupedEvent as RenderUpdateEvent[]);
						break;
					case 'TrackUpdate':
						this.handleTrackUpdates(groupedEvent as TrackUpdateEvent[]);
						break;
					case 'SelectionChange':
						this.handleSelectionChanges(groupedEvent as SelectionChangeEvent[]);
						break;
					default:
						console.warn(`Unknown event: ${eventType as string}`);
				}
			}
		}
	}

	// =========================================================================
	// Renderer
	// =========================================================================

	public registerRenderer(renderer: Renderer): void {
		renderer.setSize(this._width, this._height);
		this._renderer.push(renderer);
	}

	private handleRenderUpdates(events: RenderUpdateEvent[]): void {
		this._renderer.forEach((renderer) => {
			renderer.render(events);
		});
	}

	// =========================================================================
	// Tracking
	// =========================================================================

	public watchEntity(
		entity: Entity,
		toTrackMixins: TRustEnumKeyArray<TrackableMixinType>[],
		callback: TWatchEntityCallback
	): string | null {
		// Enable tracking of entity in composition
		const success = this.trackEntity(
			entity,
			toTrackMixins.map((v) => ({ type: v }))
		);

		// Register callback
		if (success) {
			const callbackId = shortId();
			let callbacks = this._watchEntityCallbacks.get(entity);
			if (!callbacks) {
				callbacks = new Map<string, TWatchEntityCallback>();
				this._watchEntityCallbacks.set(entity, callbacks);
			}
			callbacks.set(callbackId, callback);
			return callbackId;
		}

		return null;
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

	private trackEntity(entity: Entity, toTrackMixins: TrackableMixinType[]): boolean {
		return this._compositionHandle.trackEntity(entity, toTrackMixins);
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

	public onSelectionChange(callback: TOnSelectionChangeCallback): string {
		const callbackId = shortId();
		this._onSelectionChangeCallbacks.set(callbackId, callback);
		return callbackId;
	}

	public unregisterOnSelectionChangeCallback(callbackId: string): void {
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
	// Paint
	// =========================================================================

	public registerPaint(paint: Paint): Entity {
		return this._compositionHandle.spawnPaint(paint);
	}

	// =========================================================================
	// Entity Creation
	// =========================================================================

	public createRectangle(
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
		const paintId = this.registerPaint({
			type: 'Solid',
			blendMode: 'Normal',
			color,
			isVisible: true,
			opacity: 1
		});
		return this._compositionHandle.spawnRectangleNode(
			{
				compositionMixin: {
					isVisible: true,
					isLocked: false
				},
				dimension: {
					width: Math.round(width),
					height: Math.round(height)
				},
				relativeTransform: mat3(vec3(1, 0, 0), vec3(0, 1, 0), vec3(x, y, 1)),
				blendMixin: {
					blendMode: 'Normal',
					opacity: 1,
					isMask: false
				},
				rectangleCornerMixin: {
					bottomLeftRadius: 20,
					bottomRightRadius: 0,
					topLeftRadius: 0,
					topRightRadius: 0
				},
				fill: {
					paints: [paintId]
				}
			} as RectangleNodeBundle,
			parentId
		);
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

	public toString(): string | null {
		return this._compositionHandle.toString() ?? null;
	}
}

export interface TCompositionConfig {
	width: number;
	height: number;
	dtif?: DTIFComposition;
	isCallbackBased?: boolean;
}

type TWatchEntityCallback = (entity: Entity, changes: MixinChange[]) => void;
type TOnSelectionChangeCallback = (selected: Entity[]) => void;
