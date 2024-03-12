import type { COMP } from '@dyn/comp-dtif';

export function mapFigmaTransformToRotation(transform: Transform): COMP.Degree {
	return Math.atan2(transform[0][1], transform[0][0]) * (180 / Math.PI);
}