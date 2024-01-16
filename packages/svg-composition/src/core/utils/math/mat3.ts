import type { Mat3 } from '@/rust/dyn_svg_composition_api/bindings';

import type { Vec3 } from './vec3';

// | a d tx |
// | b e ty |
// | c f j |
// [ a, b, c, d, e, f, tx, ty, i ]
export function mat3(xAxis: Vec3, yAxis: Vec3, zAxis: Vec3): Mat3 {
	return [...xAxis, ...yAxis, ...zAxis];
}
