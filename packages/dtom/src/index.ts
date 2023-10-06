import init, { add } from '@rust/dyn-dtom';
import wasm from '@rust/dyn-dtom/bg.wasm';

export async function greetRust(): Promise<string> {
	// @ts-expect-error - "wasm" needs to be loaded async
	const wasmInstance: unknown = await wasm();
	await init(wasmInstance);
	return `Hello Rust ${add(3, 4)}`;
}
