import { JsCompositionHandle } from '@/rust/dyn_composition_api';
import type {
	AnyInputEvent,
	CoreInputEvent,
	DTIFComposition,
	Entity,
	InteractionInputEvent,
	OutputEvent,
	RectangleNodeBundle,
	RenderUpdateEvent
} from '@/rust/dyn_composition_api/bindings';

import { groupByType, mat3, vec3 } from '../helper';
import type { Renderer } from '../render';

export class Composition {
	private _compositionHandle: JsCompositionHandle;

	private _renderer: Renderer[] = [];

	protected _width: number;
	protected _height: number;

	protected _eventQueue: AnyInputEvent[] = [];

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
						relativeTransform: mat3(vec3(1, 0, 0), vec3(0, 1, 0), vec3(0, 0, 1))
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
	// WASM interface
	// =========================================================================

	public onWasmEvents(events: OutputEvent[]): this {
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
						break;
				}
			}
		}
		return this;
	}

	private onRenderUpdate(events: RenderUpdateEvent[]): this {
		this._renderer.forEach((renderer) => renderer.render(events));
		return this;
	}

	// =========================================================================
	// Interface
	// =========================================================================

	public setSize(width: number, height: number): this {
		this._width = width;
		this._height = height;
		this._renderer.forEach((renderer) => renderer.setSize(width, height));
		return this;
	}

	public registerRenderer(renderer: Renderer): this {
		renderer.setSize(this._width, this._height);
		this._renderer.push(renderer);
		return this;
	}

	public update(): void {
		this._compositionHandle.update(this._eventQueue);
		this._eventQueue = [];
	}

	public emitCoreEvents(events: CoreInputEvent[]) {
		this._eventQueue.push({ type: 'Core', events });
	}

	public emitInteractionEvents(events: InteractionInputEvent[]) {
		this._eventQueue.push({ type: 'Interaction', events });
	}

	public createRectangle(config: { x: number; y: number; width: number; height: number }): Entity {
		const { x, y, width, height } = config;
		return this._compositionHandle.spawn_rectangle_node({
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
			}
		} as RectangleNodeBundle);
	}

	public moveEntity(entity: Entity, dx: number, dy: number): void {
		this.emitCoreEvents([{ type: 'EntityMoved', entity, dx, dy }]);
	}

	public setEntityPosition(entity: Entity, x: number, y: number): void {
		this.emitCoreEvents([{ type: 'EntitySetPosition', entity, x, y }]);
	}

	public destory(): void {
		this._renderer.forEach((renderer) => renderer.destroy());
	}
}

export interface TCompositionConfig {
	width: number;
	height: number;
	dtif?: DTIFComposition;
}

export interface TWorldIds {
	mainWorldId: number;
	renderWorldId: number;
}
