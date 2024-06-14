import { isDtif } from '../dtif';
import type { TMdtifCanvas } from './types';

export function isMdtif(value: unknown): value is TMdtifCanvas {
	if (typeof value !== 'object' || value == null) {
		return false;
	}
	const obj = value as Partial<TMdtifCanvas>;
	return obj.extension === 'MDTIF' && isDtif(value);
}
