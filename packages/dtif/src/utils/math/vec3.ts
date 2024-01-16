export function vec3(x: number, y: number, z: number): Vec3 {
	return [x, y, z];
}

// Temp hardcoded Vec3 type as its not yet referenced in type exported by specta
// and thus not exported by default
export type Vec3 = [number, number, number];
