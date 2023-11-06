import init, { type InitInput } from '@/rust/dyn_composition_api';
import wasm from '@/rust/dyn_composition_api/bg.wasm';

export * from './helper';

export async function initWasm(): Promise<void> {
	// https://www.npmjs.com/package/@rollup/plugin-wasm#using-with-wasm-bindgen-and-wasm-pack
	// @ts-expect-error - "wasm" needs to be loaded async
	const wasmInstance: InitInput = await wasm();
	await init(wasmInstance);
}
