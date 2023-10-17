import init, { poll_js_event_queue, Editor as RustEditor } from '@rust/dyn-dtom';
import wasm from '@rust/dyn-dtom/bg.wasm';

export async function initWasm(): Promise<void> {
	// https://www.npmjs.com/package/@rollup/plugin-wasm#using-with-wasm-bindgen-and-wasm-pack
	// @ts-expect-error - "wasm" needs to be loaded async
	const wasmInstance: unknown = await wasm();
	await init(wasmInstance);
}

export class Editor {
	private _rustEditor: RustEditor;

	constructor() {
		this._rustEditor = new RustEditor();
	}

	public createRect(): void {
		this._rustEditor.create_rect();
	}

	public update(): void {
		this._rustEditor.update();
		this.pollAndTriggerCallbacks();
	}

	private pollAndTriggerCallbacks(): void {
		const events = JSON.parse(poll_js_event_queue() as string);
		console.log('pollAndTriggerCallbacks', { events });
	}
}

export function editorFactory(): Editor {
	return new Editor();
}

export * from '@rust/dyn-dtom';
