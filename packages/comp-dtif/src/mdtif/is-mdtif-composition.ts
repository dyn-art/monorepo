import { isDtifComposition } from '../dtif';
import type { TMdtifComposition } from './types';

export function isMdtifComposition(value: unknown): value is TMdtifComposition {
	if (typeof value !== 'object' || value == null) {
		return false;
	}
	const obj = value as Partial<TMdtifComposition>;
	return obj.extension === 'MDTIF' && isDtifComposition(value);
}
