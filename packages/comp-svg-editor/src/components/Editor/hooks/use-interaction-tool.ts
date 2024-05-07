import React from 'react';
import type { COMP } from '@dyn/comp-dtif';
import type { Composition } from '@dyn/comp-svg-builder';

import { useOutputEvent } from './use-output-event';

export function useInteractionTool(composition: Composition): COMP.InteractionTool {
	const [interactionTool, setInteractionTool] = React.useState<COMP.InteractionTool>({
		type: 'Select'
	});
	useOutputEvent(
		composition,
		'InteractionToolChange',
		(event) => {
			setInteractionTool(event.interactionTool);
		},
		[]
	);
	return interactionTool;
}
