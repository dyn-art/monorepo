import { hasFeatures } from '../has-features';
import type { TEnforceFeatures, TFeatureKeys, TSelectFeatures, TState } from '../types';

export const FAILED_TO_LOAD_IDENTIFIER = undefined;

export interface StorageInterface<GValue> {
	save: (key: string, value: GValue) => Promise<boolean>;
	load: (key: string) => Promise<GValue | typeof FAILED_TO_LOAD_IDENTIFIER>;
	delete: (key: string) => Promise<boolean>;
}

// TODO: Think about sync implementation of Persist
export function withPersist<GValue, GSelectedFeatureKeys extends TFeatureKeys<GValue>[]>(
	state: TState<GValue, TEnforceFeatures<GSelectedFeatureKeys, ['set', 'listen']>>,
	storage: StorageInterface<GValue>,
	key: string
): TState<GValue, [...GSelectedFeatureKeys, 'persist']> {
	if (hasFeatures(state, ['set', 'listen'])) {
		const persistFeature: TSelectFeatures<GValue, ['persist']> = {
			async persist() {
				let success = false;

				// Load persisted value or store inital value
				const persistedValue = await storage.load(key);
				if (persistedValue !== FAILED_TO_LOAD_IDENTIFIER) {
					state.set(persistedValue);
					success = true;
				} else {
					success = await storage.save(key, state._value);
				}

				// Setup listener
				state.listen(async (value) => {
					await storage.save(key, value);
				});

				return success;
			},
			async deletePersisted() {
				return storage.delete(key);
			}
		};

		// Merge existing features from the state with the new persist feature
		const _state = Object.assign(state, persistFeature);

		return _state as TState<GValue, [...GSelectedFeatureKeys, 'persist']>;
	}

	throw Error('State must have "set" and "listen" features to use withPersist');
}
