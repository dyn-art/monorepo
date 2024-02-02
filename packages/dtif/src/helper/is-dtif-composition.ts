import type { COMP } from '../comp';

export function isDTIFComposition(value: unknown): value is COMP.DTIFComposition {
	if (typeof value !== 'object' || value == null) {
		return false;
	}
	const obj = value as Partial<COMP.DTIFComposition>;
	return (
		typeof obj.name === 'string' &&
		typeof obj.width === 'number' &&
		typeof obj.height === 'number' &&
		typeof obj.rootNodeId === 'number' &&
		obj.nodes != null &&
		typeof obj.nodes === 'object' &&
		Object.values(obj.nodes).every((node) => typeof node === 'object') &&
		obj.paints != null &&
		typeof obj.paints === 'object' &&
		Object.values(obj.paints).every((paint) => typeof paint === 'object')
	);
}
