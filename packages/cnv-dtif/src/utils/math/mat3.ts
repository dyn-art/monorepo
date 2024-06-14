import type { CNV } from '../../cnv';

// | a d tx |
// | b e ty |
// | c f j |
// [ a, b, c, d, e, f, tx, ty, i ]
export function mat3(xAxis: CNV.Vec3, yAxis: CNV.Vec3, zAxis: CNV.Vec3): CNV.Mat3 {
	return [...xAxis, ...yAxis, ...zAxis];
}
