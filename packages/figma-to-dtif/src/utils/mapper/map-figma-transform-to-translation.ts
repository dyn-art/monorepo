import type { ARB } from '@dyn/arb-dtif';

export function mapFigmaTransformToTranslation(transform: Transform): ARB.Vec2 {
	return [transform[0][2], transform[1][2]];
}
