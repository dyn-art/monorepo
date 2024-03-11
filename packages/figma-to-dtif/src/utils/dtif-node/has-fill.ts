import type { COMP } from '@dyn/dtif';

export function hasDtifFill(obj: unknown): obj is { fill: COMP.FillMixin } {
	return obj != null && typeof obj === 'object' && 'fill' in obj;
}
