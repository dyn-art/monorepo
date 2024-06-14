import type { CNV } from '@dyn/cnv-dtif';

export function mapFigmaTransformToRotation(transform: Transform): CNV.Angle {
	return Math.atan2(transform[0][1], transform[0][0]) * (180 / Math.PI) * -1;
}
