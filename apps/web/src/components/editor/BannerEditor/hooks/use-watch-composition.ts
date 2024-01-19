import React from 'react';
import type { Composition, CompositionChange, TRustEnumKeyArray } from '@dyn/svg-composition';

export function useWatchComposition<T extends TCompositionChangeKey[]>(
	composition: Composition
): TCombinedMixin<T> {
	const initialState: TCombinedMixin<T> = {};
	const [changes, dispatch] = React.useReducer(changesReducer<T>, initialState);

	React.useEffect(() => {
		const unwatch = composition.watchComposition((changesArray) => {
			for (const change of changesArray) {
				dispatch(change);
			}
		});

		return () => {
			unwatch();
		};
	}, [composition]);

	return changes;
}

function changesReducer<T extends TCompositionChangeKey[]>(
	state: TCombinedMixin<T>,
	action: CompositionChange
): TCombinedMixin<T> {
	const { type, ...change } = action;
	return { ...state, [type]: change };
}

type TCombinedMixin<T extends TCompositionChangeKey[]> = {
	[K in T[number]]?: Extract<CompositionChange, { type: K }>;
};
type TCompositionChangeKey = TRustEnumKeyArray<CompositionChange>;
