import type { Mat3, Vec2 } from '@/rust/dyn_svg_composition_api/bindings';

export function vec2(x: number, y: number): Vec2 {
	return [x, y];
}

export function vec3(x: number, y: number, z: number): Vec3 {
	return [x, y, z];
}

// | a d tx |
// | b e ty |
// | c f j |
// [ a, b, c, d, e, f, tx, ty, i ]
export function mat3(xAxis: Vec3, yAxis: Vec3, zAxis: Vec3): Mat3 {
	return [...xAxis, ...yAxis, ...zAxis];
}

// Temp hardcoded Vec3 type as its not yet referenced in type exported by specta
// and thus not exported by default
type Vec3 = [number, number, number];
