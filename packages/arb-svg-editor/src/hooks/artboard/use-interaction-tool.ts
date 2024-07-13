import React from 'react';
import type { ARB } from '@dyn/arb-dtif';
import type { Artboard } from '@dyn/arb-svg-builder';

import { useOutputEvent } from './use-output-event';

export function useInteractionTool(artboard: Artboard): ARB.InteractionTool {
	const [interactionTool, setInteractionTool] = React.useState<ARB.InteractionTool>({
		type: 'Select'
	});
	useOutputEvent(
		artboard,
		'InteractionToolChange',
		(event) => {
			setInteractionTool(event.interactionTool);
		},
		[]
	);
	return interactionTool;
}
