import { CompositionApp as RustCompositionApp } from '@rust/dyn-dtom';
import type {
	Entity,
	FromJsEvent,
	RectangleNodeBundle,
	ToJsEvent,
	WorldIds
} from '@rust/dyn-dtom/bindings';
import type { TComposition } from '@dyn/types/dtif';

import { TEST_COMPOSITION_1 } from '../../test-data';
import {
	enqueueJsEvents,
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
		this._rustComposition = new RustCompositionApp(TEST_COMPOSITION_1); // TODO: Map TComposition to DTIFComposition
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

	public static onWasmEvents(worldId: number, events: ToJsEvent[]): void {
		Composition._INSTANCES.find((instance) => instance.hasWorldId(worldId))?.onWasmEvents(events);
	}

	public onWasmEvents(events: ToJsEvent[]): this {
		const groupedEvents: GroupedRustEnums<ToJsEvent> = transformRustEnumArrayToObject(events);

		// Process grouped events
		for (const eventType in groupedEvents) {
			const eventGroup = groupedEvents[eventType as RustEnumKeys<ToJsEvent>];
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

	private onRenderUpdate(events: ToJsEvent['RenderUpdate'][]): this {
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
		enqueueJsEvents(this.worldIds.mainWorldId, this._eventQueue);
		this._eventQueue = [];
		this._rustComposition.update();
	}

	public createRectangle(bundle: RectangleNodeBundle): Entity {
		return this._rustComposition.spawn_rectangle(bundle);
	}

	public moveEntity(entity: Entity, dx: number, dy: number): void {
		this._eventQueue.push({ EntityMoved: { entity, dx, dy } });
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
