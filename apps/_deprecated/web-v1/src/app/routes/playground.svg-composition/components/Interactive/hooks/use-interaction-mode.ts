import React from 'react';
import { Composition, InteractionModeForFrontend } from '@dyn/svg-composition';

export function useInteractionMode(composition?: Composition): InteractionModeForFrontend {
	const [interactionMode, setInteractionMode] = React.useState<InteractionModeForFrontend>({
		type: 'None'
	});

	React.useEffect(() => {
		if (composition) {
			const unwatch = composition.onInteractionModeChange((interactionMode) => {
				setInteractionMode(interactionMode);
			});
			return () => {
				unwatch();
			};
		}
	}, [composition]);

	return interactionMode;
}
