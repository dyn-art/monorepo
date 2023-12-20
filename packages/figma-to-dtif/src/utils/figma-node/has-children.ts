export function hasChildrenFigma(obj: unknown): obj is ChildrenMixin {
	return obj != null && typeof obj === 'object' && 'children' in obj;
}
