import type { TEnforceFeatures, TFeatureKeys, TSelectFeatures, TState } from '../types';

export function withUndo<GValue, GSelectedFeatureKeys extends TFeatureKeys<GValue>[]>(
	state: TState<GValue, TEnforceFeatures<GSelectedFeatureKeys, ['set', 'listen']>>
): TState<GValue, ['undo', ...GSelectedFeatureKeys]> {
	const undoFeature: TSelectFeatures<GValue, ['undo']> = {
		_history: [state._value],
		undo(this: TState<GValue, ['undo', 'set', 'listen']>) {
			const newValue = this._history.pop();
			if (newValue != null) {
				this.set(newValue);
			}
		}
	};

	const _state: TState<GValue, ['set', 'listen', 'undo']> = Object.assign(
		state,
		undoFeature
	) as any;
	_state.listen((value) => {
		if (_state._history[_state._history.length - 1] !== value) {
			_state._history.push(value);
		}
	});

	return _state as unknown as TState<GValue, ['undo', ...GSelectedFeatureKeys]>;
}
