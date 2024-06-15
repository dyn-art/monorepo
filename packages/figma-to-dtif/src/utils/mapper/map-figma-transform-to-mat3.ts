import type { ARB } from '@dyn/arb-dtif';

export function mapFigmaTransformToMat3(transform: Transform): ARB.Mat3 {
	const [a, c, e] = transform[0];
	const [b, d, f] = transform[1];

	// Create a Mat3 array
	// The third row is [0, 0, 1] for 2D transformations
	const mat3: ARB.Mat3 = [a, b, 0, c, d, 0, e, f, 1];

	return mat3;
}
