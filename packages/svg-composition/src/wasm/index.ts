import init, { type InitInput } from '@/rust/dyn_svg_composition_api';
import wasm from '@/rust/dyn_svg_composition_api/bg.wasm';

export * from './types';

export async function initWasm(): Promise<void> {
	// https://www.npmjs.com/package/@rollup/plugin-wasm#using-with-wasm-bindgen-and-wasm-pack
	// @ts-expect-error - "wasm" needs to be loaded async
	const wasmInstance: InitInput = await wasm();
	await init(wasmInstance);
}
