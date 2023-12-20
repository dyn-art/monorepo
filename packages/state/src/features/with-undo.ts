import { hasFeatures } from '../has-features';
import type { TEnforceFeatures, TFeatureKeys, TSelectFeatures, TState } from '../types';

export function withUndo<GValue, GSelectedFeatureKeys extends TFeatureKeys<GValue>[]>(
	state: TState<GValue, TEnforceFeatures<GSelectedFeatureKeys, ['set', 'listen']>>,
	historyLimit = 50
): TState<GValue, ['undo', ...GSelectedFeatureKeys]> {
	if (hasFeatures(state, ['set', 'listen'])) {
		const undoFeature: TSelectFeatures<GValue, ['undo']> = {
			_history: [state._value],
			undo(this: TState<GValue, ['undo', 'set', 'listen']>) {
				this._history.pop(); // Pop current value
				const newValue = this._history.pop(); // Pop previous value
				if (newValue != null) {
					this.set(newValue);
				}
			}
		};

		// Merge existing features from the state with the new undo feature
		const _state = Object.assign(state, undoFeature);

		_state.listen((value) => {
			// Maintaining the history stack size
			if (_state._history.length >= historyLimit) {
				_state._history.shift(); // Remove oldest state
			}

			_state._history.push(value);
		});

		return _state as TState<GValue, ['undo', ...GSelectedFeatureKeys]>;
	}

	throw Error('State must have "set" and "listen" features to use withUndo');
}