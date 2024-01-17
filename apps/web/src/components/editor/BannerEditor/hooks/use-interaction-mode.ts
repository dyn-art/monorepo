import React from 'react';
import type { Composition, InteractionModeForFrontend } from '@dyn/svg-composition';

export function useInteractionMode(composition?: Composition): InteractionModeForFrontend {
	const [interactionMode, setInteractionMode] = React.useState<InteractionModeForFrontend>({
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
