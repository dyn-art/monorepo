import type { COMP } from '../comp';

export function isDTIFComposition(value: unknown): value is COMP.CompDtif {
	if (typeof value !== 'object' || value == null) {
		return false;
	}
	const obj = value as Partial<COMP.CompDtif>;
	return (
		typeof obj.name === 'string' &&
		Array.isArray(obj.size) &&
		// eslint-disable-next-line @typescript-eslint/no-unnecessary-condition -- Necessary here as we don't know how long the size array might be
		obj.size.length === 2 &&
		typeof obj.size[0] === 'number' &&
		typeof obj.size[1] === 'number' &&
		typeof obj.rootNodeId === 'number' &&
		obj.nodes != null &&
		typeof obj.nodes === 'object' &&
		Object.values(obj.nodes).every((node) => typeof node === 'object')
	);
}
