import type { COMP } from '../../comp';

// | a d tx |
// | b e ty |
// | c f j |
// [ a, b, c, d, e, f, tx, ty, i ]
export function mat3(xAxis: COMP.Vec3, yAxis: COMP.Vec3, zAxis: COMP.Vec3): COMP.Mat3 {
	return [...xAxis, ...yAxis, ...zAxis];
}
