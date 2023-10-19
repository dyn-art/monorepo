import { Editor as RustEditor } from '@rust/dyn-dtom';

export class Editor {
	private _rustEditor: RustEditor;
	private _worldIds: number[] = [];

	private static INSTANCES: Editor[] = [];

	constructor() {
		this._rustEditor = new RustEditor();
		this._worldIds = this._rustEditor.get_world_ids();
		Editor.INSTANCES.push(this);
	}

	public static onWasmEvent(worldId: number, data: unknown): void {
		Editor.INSTANCES.find((instance) => instance.hasWorldId(worldId))?.onWasmEvent(data);
	}

	public onWasmEvent(data: unknown) {
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

export function editorFactory(): Editor {
	return new Editor();
}
