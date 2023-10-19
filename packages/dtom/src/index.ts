import init, { Editor as RustEditor } from '@rust/dyn-dtom';
import wasm from '@rust/dyn-dtom/bg.wasm';

export async function initWasm(): Promise<void> {
	// https://www.npmjs.com/package/@rollup/plugin-wasm#using-with-wasm-bindgen-and-wasm-pack
	// @ts-expect-error - "wasm" needs to be loaded async
	const wasmInstance: unknown = await wasm();
	await init(wasmInstance);
}

(globalThis as any).receiveRustEvents = function onNewWasmEvents(data: unknown) {
	Editor.onWasmEvent(data);
};

export class Editor {
	private _rustEditor: RustEditor;

	private static INSTANCES: Editor[] = [];

	constructor() {
		this._rustEditor = new RustEditor();
		Editor.INSTANCES.push(this);
	}

	public static onWasmEvent(data: unknown): void {
		Editor.INSTANCES.forEach((instance) => {
			// TODO:
			console.log('pollAndTriggerCallbacks', { data });
		});
	}

	public createRect(): void {
		this._rustEditor.create_rect();
	}

	public update(): void {
		this._rustEditor.update();
	}
}

export function editorFactory(): Editor {
	return new Editor();
}

export * from '@rust/dyn-dtom';
