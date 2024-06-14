import type { ARB } from '@dyn/arb-dtif';

export function mapFigmaTransformToRotation(transform: Transform): ARB.Angle {
	return Math.atan2(transform[0][1], transform[0][0]) * (180 / Math.PI) * -1;
}
