import { Editor as RustEditor } from '@rust/dyn-dtom';

import type { Renderer } from './render';

export class Editor {
	private _rustEditor: RustEditor;
	private _worldIds: number[] = [];

	private static _INSTANCES: Editor[] = [];

	private _renderer: Renderer;

	constructor(renderer: Renderer) {
		this._rustEditor = new RustEditor();
		this._worldIds = this._rustEditor.get_world_ids();
		this._renderer = renderer;
		Editor._INSTANCES.push(this);
	}

	public static onWasmEvent(worldId: number, data: unknown): void {
		Editor._INSTANCES.find((instance) => instance.hasWorldId(worldId))?.onWasmEvent(data);
	}

	public onWasmEvent(data: unknown) {
		this._renderer.render();
		// TODO:
		console.log('onWasmEvent', { data });
	}

	public createRect(): void {
		this._rustEditor.create_rect();
	}

	public hasWorldId(worldId: number): boolean {
		return this._worldIds.includes(worldId);
	}

	public update(): void {
		this._rustEditor.update();
	}
}
