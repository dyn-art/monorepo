import React from 'react';
import type { COMP } from '@dyn/comp-dtif';
import type { Composition } from '@dyn/comp-svg-builder';

import { useOutputEvent } from './use-output-event';

export function useInteractionMode(composition: Composition): COMP.InteractionModeLabel {
	const [interactionModeLabel, setInteractionModeLabel] =
		React.useState<COMP.InteractionModeLabel>('None');
	useOutputEvent(
		composition,
		'InteractionModeChange',
		(event) => {
			setInteractionModeLabel(event.interactionMode);
		},
		[]
	);
	return interactionModeLabel;
}
