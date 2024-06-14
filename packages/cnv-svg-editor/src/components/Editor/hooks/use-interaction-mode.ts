import React from 'react';
import type { CNV } from '@dyn/cnv-dtif';
import type { Canvas } from '@dyn/cnv-svg-builder';

import { useOutputEvent } from './use-output-event';

export function useInteractionMode(canvas: Canvas): CNV.InteractionModeLabel {
	const [interactionModeLabel, setInteractionModeLabel] =
		React.useState<CNV.InteractionModeLabel>('None');
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
