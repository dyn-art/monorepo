import {
	FAILED_TO_LOAD_IDENTIFIER,
	withPersist,
	type StorageInterface,
	type TEnforceFeatures,
	type TFeatureKeys,
	type TState
} from 'feature-state';

class FigmaClientStorageInterface<GValue> implements StorageInterface<GValue> {
	async save(key: string, value: GValue): Promise<boolean> {
		await figma.clientStorage.setAsync(key, value);
		return true;
	}

	async load(key: string): Promise<GValue | typeof FAILED_TO_LOAD_IDENTIFIER> {
		const value = await figma.clientStorage.getAsync(key);
		return value !== undefined ? value : FAILED_TO_LOAD_IDENTIFIER;
	}

	async delete(key: string): Promise<boolean> {
		await figma.clientStorage.deleteAsync(key);
		return true;
	}
}

export function withPersistFigmaClientStorage<
	GValue,
	GSelectedFeatureKeys extends TFeatureKeys<GValue>[]
>(
	state: TState<GValue, TEnforceFeatures<GSelectedFeatureKeys, ['set', 'listen']>>,
	key: string
): TState<GValue, [...GSelectedFeatureKeys, 'persist']> {
	return withPersist(state, new FigmaClientStorageInterface<GValue>(), key);
}
