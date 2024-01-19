import React from 'react';
import type { Composition } from '@dyn/svg-composition';
import type { CompositionChange } from '@dyn/svg-composition/dist/types/rust_modules/dyn_svg_composition_api/bindings';

export function useWatchComposition(composition: Composition): CompositionChange | null {
	const [change, setChange] = React.useState<CompositionChange | null>(null);

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
