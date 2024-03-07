import type * as COMP from './_gen/bindings';

declare module './_gen/bindings' {
	// Temp hardcoded Vec3 type as its not yet referenced in type exported by specta
	// and thus not exported by default
	export type Vec3 = [number, number, number];
}

export { COMP };
