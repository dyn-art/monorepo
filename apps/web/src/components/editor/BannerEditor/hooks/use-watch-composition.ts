import React from 'react';
import type { COMP, Composition } from '@dyn/svg-composition';

export function useWatchComposition(composition: Composition): COMP.CompositionChange | null {
	const [change, setChange] = React.useState<COMP.CompositionChange | null>(null);

	React.useEffect(() => {
		const unwatch = composition.watchComposition((newChange) => {
			setChange(newChange);
		});

		return () => {
			unwatch();
		};
	}, [composition]);

	return change;
}
