import type { TEnforceFeatures, TFeatureKeys, TState } from '../types';
import { withPersist, type StorageInterface } from './with-persist';

class LocalStorageInterface<GValue> implements StorageInterface<GValue> {
	save(key: string, value: GValue): void {
		// @ts-expect-error
		localStorage.setItem(key, JSON.stringify(value));
	}

	load(key: string): GValue | null {
		// @ts-expect-error
		const item = localStorage.getItem(key);
		return item ? (JSON.parse(item) as GValue) : null;
	}
}

export function withPersistLocalStorage<
	GValue,
	GSelectedFeatureKeys extends TFeatureKeys<GValue>[]
>(
	state: TState<GValue, TEnforceFeatures<GSelectedFeatureKeys, ['set', 'listen']>>,
	key: string
): TState<GValue, [...GSelectedFeatureKeys, 'persist']> {
	return withPersist(state, new LocalStorageInterface<GValue>(), key);
}
