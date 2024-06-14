import { isDtif } from '../dtif';
import type { TMdtifArtboard } from './types';

export function isMdtif(value: unknown): value is TMdtifArtboard {
	if (typeof value !== 'object' || value == null) {
		return false;
	}
	const obj = value as Partial<TMdtifArtboard>;
	return obj.extension === 'MDTIF' && isDtif(value);
}
