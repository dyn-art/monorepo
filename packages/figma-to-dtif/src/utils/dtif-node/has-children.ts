import type { COMP } from '@dyn/dtif';

export function hasChildrenDTIF(obj: unknown): obj is { children: COMP.ChildrenMixin } {
	return obj != null && typeof obj === 'object' && 'children' in obj;
}
