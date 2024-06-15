import type { ARB } from '../../arb';
import { mat3 } from './mat3';
import { vec3 } from './vec3';

export function createTransformMatrix(x: number, y: number, angleDegrees: number): ARB.Mat3 {
	const angleRadians = (angleDegrees * Math.PI) / 180; // Convert angle to radians

	return mat3(
		vec3(Math.cos(angleRadians), -Math.sin(angleRadians), 0),
		vec3(Math.sin(angleRadians), Math.cos(angleRadians), 0),
		vec3(x, y, 1)
	);
}
