import { describe, expect, it } from 'vitest';

import { createState } from '../create-state';
import { withUndo } from './with-undo';

describe('withUndo extension tests', () => {
	it('should allow undoing the last set operation', () => {
		// Prepare
		const state = withUndo(createState(10));

		// Act
		state.set(20);
		state.undo();

		// Assert
		expect(state.get()).toBe(10);
	});

	it('should handle multiple undos correctly', () => {
		// Prepare
		const state = withUndo(createState('initial'));

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
		const state = withUndo(createState(10));

		// Act
		state.undo();

		// Assert
		expect(state.get()).toBe(10);
	});

	it('should only record distinct consecutive values for undo', () => {
		// Prepare
		const state = withUndo(createState(10));

		// Act
		state.set(10); // Same as initial, should not be recorded
		state.set(20);
		state.set(20); // Duplicate, should not be recorded again
		state.undo();

		// Assert
		expect(state.get()).toBe(10);
	});
});
