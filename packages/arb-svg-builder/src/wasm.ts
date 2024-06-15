import init, { type InitInput } from '@/rust/dyn-arb-svg-builder-api';
import wasm from '@/rust/dyn-arb-svg-builder-api/bg.wasm';

export async function initWasm(): Promise<void> {
	// https://www.npmjs.com/package/@rollup/plugin-wasm#using-with-wasm-bindgen-and-wasm-pack
	// @ts-expect-error - Needs to be loaded async
	const wasmInstance: InitInput = await wasm();
	await init(wasmInstance);
}
