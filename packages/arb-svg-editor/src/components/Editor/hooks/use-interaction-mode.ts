import React from 'react';
import type { ARB } from '@dyn/arb-dtif';
import type { Artboard } from '@dyn/arb-svg-builder';

import { useOutputEvent } from './use-output-event';

export function useInteractionMode(canvas: Artboard): ARB.InteractionModeLabel {
	const [interactionModeLabel, setInteractionModeLabel] =
		React.useState<ARB.InteractionModeLabel>('None');
	useOutputEvent(
		canvas,
		'InteractionModeChange',
		(event) => {
			setInteractionModeLabel(event.interactionMode);
		},
		[]
	);
	return interactionModeLabel;
}
