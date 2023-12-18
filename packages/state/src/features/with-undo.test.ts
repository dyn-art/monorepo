import { describe, expect, it } from 'vitest';

import { createState } from '../create-state';
import { withUndo } from './with-undo';

describe('withUndo function tests', () => {
	// it('should have correct types', () => {
	// 	const state: TState<string, ['get', 'set', 'listen']> = createState('Jeff');
	// 	const stateWithUndo = withUndo(state);
	// });

	it('should allow undoing the last set operation', () => {
		// Prepare
		const state = withUndo(createState(10, false));

		// Act
		state.set(20);
		state.undo();

		// Assert
		expect(state.get()).toBe(10);
	});

	it('should handle multiple undos correctly', () => {
		// Prepare
		const state = withUndo(createState('initial', false));

		// Act
		state.set('first');
		state.set('second');
		state.undo();
		state.undo();

		// Assert
		expect(state.get()).toBe('initial');
	});

	it('should do nothing if there is nothing to undo', () => {
		// Prepare
		const state = withUndo(createState(10, false));

		// Act
		state.undo();

		// Assert
		expect(state.get()).toBe(10);
	});

	it('should only record distinct consecutive values for undo', () => {
		// Prepare
		const state = withUndo(createState(10, false));

		// Act
		state.set(10); // Same as initial, should not be recorded
		state.set(20);
		state.set(20); // Duplicate, should not be recorded again
		state.undo();

		// Assert
		expect(state.get()).toBe(10);
	});

	it('should throw an error if the state does not have required features', () => {
		// Prepare
		const incompleteState = {};

		// Act & Assert
		expect(() => withUndo(incompleteState as any)).toThrow(
			'State must have "set" and "listen" features to use withUndo'
		);
	});
});
