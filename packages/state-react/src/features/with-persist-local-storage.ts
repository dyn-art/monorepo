import {
	withPersist,
	type StorageInterface,
	type TEnforceFeatures,
	type TFeatureKeys,
	type TState
} from '@dyn/state';

class LocalStorageInterface<GValue> implements StorageInterface<GValue> {
	save(key: string, value: GValue): void {
		localStorage.setItem(key, JSON.stringify(value));
	}

	load(key: string): GValue | null {
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
