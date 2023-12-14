import type { FillMixin } from '@dyn/svg-composition/bindings';

export function hasFillDTIF(obj: unknown): obj is { fill: FillMixin } {
	return obj != null && typeof obj === 'object' && 'fill' in obj;
}
