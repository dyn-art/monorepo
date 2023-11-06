import type { TComposition } from '@dyn/types/dtif';
import { JsCompositionHandle } from '@/rust/dyn_dtom_api';
import type { Entity, InputEvent, OutputEvent } from '@/rust/dyn_dtom_api/bindings';

import { EMPTY_COMPOSITION } from '../../test-data';
import {
	transformRustEnumArrayToObject,
	type GroupedRustEnums,
	type RustEnumKeys
} from '../../wasm';
import type { Renderer } from '../render';

export class Composition {
	private _compositionHandle: JsCompositionHandle;

	private _renderer: Renderer[] = [];

	protected _width: number;
	protected _height: number;

	protected _eventQueue: InputEvent[] = [];

	constructor(config: TCompositionConfig) {
		const { width, height } = config;
		this._compositionHandle = new JsCompositionHandle(
			EMPTY_COMPOSITION,
			(events: OutputEvent[]) => {
				this.onWasmEvents(events);
			}
		);
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
		const groupedEvents: GroupedRustEnums<OutputEvent> = transformRustEnumArrayToObject(events);

		// Process grouped events
		for (const eventType in groupedEvents) {
			const eventGroup = groupedEvents[eventType as RustEnumKeys<OutputEvent>];
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

	private onRenderUpdate(events: OutputEvent['RenderUpdate'][]): this {
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

	public emitEvents(events: InputEvent[]) {
		this._eventQueue.push(...events);
	}

	public createRectangle(config: { x: number; y: number; width: number; height: number }): Entity {
		const { x, y, width, height } = config;
		return this._compositionHandle.spawn_rectangle({
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
