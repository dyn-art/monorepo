import { hasFeatures } from '../has-features';
import type { TEnforceFeatures, TFeatureKeys, TState } from '../types';

export interface StorageInterface<GValue> {
	save: (key: string, value: GValue) => void;
	load: (key: string) => GValue | null;
}

export function withPersist<GValue, GSelectedFeatureKeys extends TFeatureKeys<GValue>[]>(
	state: TState<GValue, TEnforceFeatures<GSelectedFeatureKeys, ['set', 'listen']>>,
	storage: StorageInterface<GValue>,
	key: string
): TState<GValue, [...GSelectedFeatureKeys, 'persist']> {
	if (hasFeatures(state, ['set', 'listen'])) {
		const persistedValue = storage.load(key);
		if (persistedValue !== null) {
			state.set(persistedValue);
		}

		state.listen((value) => {
			storage.save(key, value);
		});

		return state as TState<GValue, [...GSelectedFeatureKeys, 'persist']>;
	}

	throw Error('State must have "set" and "listen" features to use withPersist');
}
