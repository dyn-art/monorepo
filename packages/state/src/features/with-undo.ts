import { hasFeatures } from '../has-features';
import type { TEnforceFeatures, TFeatureKeys, TSelectFeatures, TState } from '../types';

export function withUndo<GValue, GSelectedFeatureKeys extends TFeatureKeys<GValue>[]>(
	state: TState<GValue, TEnforceFeatures<GSelectedFeatureKeys, ['set', 'listen']>>
): TState<GValue, ['undo', ...GSelectedFeatureKeys]> {
	if (hasFeatures(state, ['set', 'listen'])) {
		const undoFeature: TSelectFeatures<GValue, ['undo']> = {
			_history: [state._value],
			undo(this: TState<GValue, ['undo', 'set', 'listen']>) {
				const newValue = this._history.pop();
				if (newValue != null) {
					this.set(newValue);
				}
			}
		};

		// Merge existing features from the state with the new undo feature
		const _state = Object.assign(state, undoFeature);

		_state.listen((value) => {
			if (_state._history[_state._history.length - 1] !== value) {
				_state._history.push(value);
			}
		});

		return _state as TState<GValue, ['undo', ...GSelectedFeatureKeys]>;
	}

	throw Error('State must have "set" and "listen" features to use withUndo');
}
