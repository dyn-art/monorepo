import type { ARB } from '../../arb';

// | a d tx |
// | b e ty |
// | c f j |
// [ a, b, c, d, e, f, tx, ty, i ]
export function mat3(xAxis: ARB.Vec3, yAxis: ARB.Vec3, zAxis: ARB.Vec3): ARB.Mat3 {
	return [...xAxis, ...yAxis, ...zAxis];
}
