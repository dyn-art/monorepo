import { beforeEach, describe, expect, it } from 'vitest';

import { createState } from '../create-state';
import { withPersist, type StorageInterface } from './with-persist';

class MockStorage<GValue> implements StorageInterface<GValue> {
	private store: Record<string, GValue> = {};

	save(key: string, value: GValue): void {
		this.store[key] = value;
	}

	load(key: string): GValue | null {
		return this.store[key] || null;
	}

	clear(): void {
		this.store = {};
	}
}

describe('withPersist function tests', () => {
	let mockStorage: MockStorage<any>;

	beforeEach(() => {
		mockStorage = new MockStorage();
	});

	it('should initialize state with persisted value if available', () => {
		// Prepare
		const key = 'testKey';
		const persistedValue = 42;
		mockStorage.save(key, persistedValue);

		// Act
		const state = withPersist(createState(0, false), mockStorage, key);

		// Assert
		expect(state.get()).toBe(persistedValue);
	});

	it('should persist state changes', () => {
		// Prepare
		const key = 'testKey';
		const state = withPersist(createState(10, false), mockStorage, key);

		// Act
		state.set(20);

		// Assert
		expect(mockStorage.load(key)).toBe(20);
	});

	it('should not override state with null if no persisted value', () => {
		// Prepare
		const key = 'testKey';

		// Act
		const state = withPersist(createState(10, false), mockStorage, key);

		// Assert
		expect(state.get()).toBe(10);
	});

	it('should throw an error if the state does not have required features', () => {
		// Prepare
		const incompleteState = {};
		const key = 'testKey';

		// Act & Assert
		expect(() => withPersist(incompleteState as any, mockStorage, key)).toThrow(
			'State must have "set" and "listen" features to use withPersist'
		);
	});
});
