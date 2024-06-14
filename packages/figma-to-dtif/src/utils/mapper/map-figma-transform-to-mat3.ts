import type { CNV } from '@dyn/cnv-dtif';

export function mapFigmaTransformToMat3(transform: Transform): CNV.Mat3 {
	const [a, c, e] = transform[0];
	const [b, d, f] = transform[1];

	// Create a Mat3 array
	// The third row is [0, 0, 1] for 2D transformations
	const mat3: CNV.Mat3 = [a, b, 0, c, d, 0, e, f, 1];

	return mat3;
}
