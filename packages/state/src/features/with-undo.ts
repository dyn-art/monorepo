import type { TEnforceFeatures, TFeatureKeys, TSelectFeatures, TState } from '../types';

export function withUndo<GValue, GSelectedFeatureKeys extends TFeatureKeys<GValue>[]>(
	state: TState<GValue, TEnforceFeatures<GSelectedFeatureKeys, ['set']>>
): TState<GValue, ['undo', ...GSelectedFeatureKeys]> {
	const undoPlugin: TSelectFeatures<GValue, ['undo']> = {
		_previousValue: state._value,
		undo(this: TState<GValue, ['undo', 'set']>) {
			this.set(this._previousValue);
		}
	};
	return Object.assign(state, undoPlugin) as TState<GValue, ['undo', ...GSelectedFeatureKeys]>;
}
