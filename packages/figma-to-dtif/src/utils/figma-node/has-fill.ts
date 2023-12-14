export function hasFill(obj: unknown): obj is MinimalFillsMixin {
	return obj != null && typeof obj === 'object' && 'fills' in obj;
}
