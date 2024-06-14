import type { CNV } from '@dyn/cnv-dtif';

export function mapFigmaTransformToTranslation(transform: Transform): CNV.Vec2 {
	return [transform[0][2], transform[1][2]];
}
