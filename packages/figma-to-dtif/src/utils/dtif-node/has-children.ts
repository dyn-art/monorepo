import type { TChildrenMixin } from '@dyn/dtif';

export function hasChildrenDTIF(obj: unknown): obj is { children: TChildrenMixin } {
	return obj != null && typeof obj === 'object' && 'children' in obj;
}
