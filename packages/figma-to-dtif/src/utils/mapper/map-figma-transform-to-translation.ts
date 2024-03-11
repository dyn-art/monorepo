import type { COMP } from '@dyn/comp-dtif';

export function mapFigmaTransformToTranslation(transform: Transform): COMP.Vec2 {
	return [transform[0][2], transform[1][2]];
}
