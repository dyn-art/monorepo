import { JsCompositionHandle } from '@/rust/dyn_composition_api';
import type {
	AnyInputEvent,
	CoreInputEvent,
	DTIFComposition,
	Entity,
	InteractionInputEvent,
	OutputEvent,
	Paint,
	RectangleNodeBundle,
	RenderUpdateEvent
} from '@/rust/dyn_composition_api/bindings';

import { groupByType, mat3, vec3 } from '../helper';
import type { Renderer } from '../render';

export class Composition {
	private readonly _compositionHandle: JsCompositionHandle;

	private _renderer: Renderer[] = [];

	protected _width: number;
	protected _height: number;

	private _eventQueue: AnyInputEvent[] = [];

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
			}
		} = config;
		this._compositionHandle = new JsCompositionHandle(dtif, (events: OutputEvent[]) => {
			this.onWasmEvents(events);
		});
		this._width = width;
		this._height = height;
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

	public onWasmEvents(events: OutputEvent[]): void {
		const groupedEvents = groupByType(events);
		for (const eventType of Object.keys(groupedEvents) as (keyof typeof groupedEvents)[]) {
			const groupedEvent = groupedEvents[eventType];
			if (groupedEvent != null) {
				switch (eventType) {
					case 'RenderUpdate':
						this.onRenderUpdate(groupedEvent);
						break;
					default:
						console.warn(`Unknown event: ${eventType as string}`);
				}
			}
		}
	}

	private onRenderUpdate(events: RenderUpdateEvent[]): this {
		this._renderer.forEach((renderer) => {
			renderer.render(events);
		});
		return this;
	}

	// =========================================================================
	// Renderer
	// =========================================================================

	public registerRenderer(renderer: Renderer): this {
		renderer.setSize(this._width, this._height);
		this._renderer.push(renderer);
		return this;
	}

	// =========================================================================
	// Cycle
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

		// Debounce: Delay update call, resetting timer on new events within debounceDelay
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
}
