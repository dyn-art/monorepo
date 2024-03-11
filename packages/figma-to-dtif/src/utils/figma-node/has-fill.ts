export function hasFigmaFill(obj: unknown): obj is MinimalFillsMixin {
	return obj != null && typeof obj === 'object' && 'fills' in obj;
}
