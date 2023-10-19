import init from '@rust/dyn-dtom';
import wasm from '@rust/dyn-dtom/bg.wasm';

export async function initWasm(): Promise<void> {
	// https://www.npmjs.com/package/@rollup/plugin-wasm#using-with-wasm-bindgen-and-wasm-pack
	// @ts-expect-error - "wasm" needs to be loaded async
	const wasmInstance: unknown = await wasm();
	await init(wasmInstance);
}
