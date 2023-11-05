import type { TComposition } from '@dyn/types/dtif';
import { CompositionApp as RustCompositionApp } from '@/rust/dyn_dtom';
import type {
	BindgenRenderToJsEvent,
	Entity,
	FromJsEvent,
	WorldIds
} from '@/rust/dyn_dtom/bindings';

import { EMPTY_COMPOSITION } from '../../test-data';
import {
	transformRustEnumArrayToObject,
	type GroupedRustEnums,
	type RustEnumKeys
} from '../../wasm';
import type { Renderer } from '../render';

export class Composition {
	private _rustComposition: RustCompositionApp;
	private _worldIds: TWorldIds;

	// Keep track of all instances of the Composition class
	// so we can pass WASM events to the correct instance based on the worldId
	private static _INSTANCES: Composition[] = [];

	private _renderer: Renderer[] = [];

	protected _width: number;
	protected _height: number;

	protected _eventQueue: FromJsEvent[] = [];

	constructor(config: TCompositionConfig) {
		const { width, height } = config;
		this._rustComposition = new RustCompositionApp(EMPTY_COMPOSITION); // TODO: Map TComposition to DTIFComposition
		this._worldIds = this.retrieveWorldIds();
		this._width = width;
		this._height = height;

		Composition._INSTANCES.push(this);
	}

	// =========================================================================
	// Getter & Setter
	// =========================================================================

	public get worldIds(): TWorldIds {
		return this._worldIds;
	}

	public get width(): number {
		return this._width;
	}

	public get height(): number {
		return this._height;
	}

	// =========================================================================
	// WASM interface
	// =========================================================================

	private retrieveWorldIds(): TWorldIds {
		const worldIds: WorldIds = this._rustComposition.get_world_ids();
		return {
			mainWorldId: worldIds.main_world_id,
			renderWorldId: worldIds.render_world_id
		};
	}

	public static onWasmEvents(worldId: number, events: BindgenRenderToJsEvent[]): void {
		Composition._INSTANCES.find((instance) => instance.hasWorldId(worldId))?.onWasmEvents(events);
	}

	public onWasmEvents(events: BindgenRenderToJsEvent[]): this {
		const groupedEvents: GroupedRustEnums<BindgenRenderToJsEvent> =
			transformRustEnumArrayToObject(events);

		// Process grouped events
		for (const eventType in groupedEvents) {
			const eventGroup = groupedEvents[eventType as RustEnumKeys<BindgenRenderToJsEvent>];
			if (eventGroup == null) {
				continue;
			}
			switch (eventType) {
				case 'RenderUpdate':
					this.onRenderUpdate(eventGroup);
					break;
				default:
					console.warn(`Unknown event: ${eventType}`);
					break;
			}
		}

		return this;
	}

	private onRenderUpdate(events: BindgenRenderToJsEvent['RenderUpdate'][]): this {
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

	public hasWorldId(worldId: number): boolean {
		return Object.values(this._worldIds).includes(worldId);
	}

	public update(): void {
		this._rustComposition.update(this._eventQueue);
		this._eventQueue = [];
	}

	public emitEvents(events: FromJsEvent[]) {
		this._eventQueue.push(...events);
	}

	public createRectangle(config: { x: number; y: number; width: number; height: number }): Entity {
		const { x, y, width, height } = config;
		return this._rustComposition.spawn_rectangle({
			node: {
				node_type: 'Rectangle'
			},
			recangle: null,
			rectangle_corner_mixin: {
				top_left_radius: 5,
				top_right_radius: 5,
				bottom_right_radius: 5,
				bottom_left_radius: 5
			},
			composition_mixin: {
				is_visible: true,
				is_locked: false
			},
			layout_mixin: {
				width: Math.round(width),
				height: Math.round(height),
				relative_transform: [1, 0, x, 0, 1, y, 0, 0, 1]
			},
			blend_mixin: {
				blend_mode: 'Normal',
				opacity: 1,
				is_mask: false
			}
		});
	}

	public moveEntity(entity: Entity, dx: number, dy: number): void {
		this.emitEvents([{ EntityMoved: { entity, dx, dy } }]);
	}

	public setEntityPosition(entity: Entity, x: number, y: number): void {
		this.emitEvents([{ EntitySetPosition: { entity, x, y } }]);
	}

	public destory(): void {
		this._renderer.forEach((renderer) => renderer.destroy());
	}
}

export interface TCompositionConfig {
	width: number;
	height: number;
	dtif?: TComposition;
}

export interface TWorldIds {
	mainWorldId: number;
	renderWorldId: number;
}
