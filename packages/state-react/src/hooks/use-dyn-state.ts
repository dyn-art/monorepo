import React from 'react';
import type { TState } from '@dyn/state';

export function useDynState<GValue>(state: TState<GValue, ['base']>): GValue {
	const [, forceRender] = React.useReducer((s) => s + 1, 0);

	React.useEffect(() => {
		const unbind = state.listen(() => {
			forceRender();
		});
		return () => {
			unbind();
		};
	}, []);

	return state._value;
}
