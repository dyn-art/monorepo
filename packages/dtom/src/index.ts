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

	private static INSTANCES: Editor[] = [];

	constructor() {
		this._rustEditor = new RustEditor();
		Editor.INSTANCES.push(this);
	}

	public static onWasmEvent(data: any): void {
		console.log('onWasmEvent', { data });
		Editor.INSTANCES.forEach((instance) => {
			instance.pollEvents();
		});
	}

	public createRect(): void {
		this._rustEditor.create_rect();
	}

	public update(): void {
		this._rustEditor.update();
		this.pollEvents();
	}

	private pollEvents(): void {
		const events = poll_js_event_queue();
		console.log('pollAndTriggerCallbacks', { events });
	}
}

export function editorFactory(): Editor {
	return new Editor();
}

export * from '@rust/dyn-dtom';
