import { isDtifComposition } from '../dtif';
import type { TMdtifComposition } from './types';

export function isMdtifComposition(value: unknown): value is TMdtifComposition {
	if (typeof value !== 'object' || value == null) {
		return false;
	}
	const obj = value as Partial<TMdtifComposition>;
	return (
		Array.isArray(obj.modificationFields) &&
		obj.modificationFields.length > 0 &&
		Object.values(obj.modificationFields).every((field) => typeof field === 'object') &&
		isDtifComposition(value)
	);
}
