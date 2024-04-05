import React from 'react';
import type { COMP } from '@dyn/dtif-comp';
import type { Composition } from '@dyn/svg-comp';

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
