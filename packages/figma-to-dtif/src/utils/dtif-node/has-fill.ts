import type { TFillMixin } from '@dyn/dtif';

export function hasFillDTIF(obj: unknown): obj is { fill: TFillMixin } {
	return obj != null && typeof obj === 'object' && 'fill' in obj;
}
