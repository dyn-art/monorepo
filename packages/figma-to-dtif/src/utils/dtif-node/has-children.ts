import type { ChildrenMixin } from '@dyn/svg-composition/bindings';

export function hasChildrenDTIF(obj: unknown): obj is { children: ChildrenMixin } {
	return obj != null && typeof obj === 'object' && 'children' in obj;
}
