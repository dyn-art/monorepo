import { isDtifComposition } from '../dtif';
import type { TMdtifComposition } from './types';

export function isMdtifComposition(value: unknown): value is TMdtifComposition {
	if (typeof value !== 'object' || value == null) {
		return false;
	}
	const obj = value as Partial<TMdtifComposition>;
	return (
		isDtifComposition(obj.template) &&
		Array.isArray(obj.modificationFields) &&
		Object.values(obj.modificationFields).every((node) => typeof node === 'object')
	);
}
