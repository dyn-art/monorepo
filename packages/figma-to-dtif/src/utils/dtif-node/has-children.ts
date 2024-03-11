import type { COMP } from '@dyn/dtif';

export function hasDtifChildren(obj: unknown): obj is { children: COMP.ChildrenMixin } {
	return obj != null && typeof obj === 'object' && 'children' in obj;
}
