import React from 'react';
import type { COMP, Composition } from '@dyn/svg-composition';

export function useInteractionMode(composition?: Composition): COMP.InteractionModeForFrontend {
	const [interactionMode, setInteractionMode] = React.useState<COMP.InteractionModeForFrontend>({
		type: 'None'
	});

	React.useEffect(() => {
		if (composition) {
			const unwatch = composition.onInteractionModeChange((_interactionMode) => {
				setInteractionMode(_interactionMode);
			});
			return () => {
				unwatch();
			};
		}
	}, [composition]);

	return interactionMode;
}
