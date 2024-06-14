import React from 'react';
import type { CNV } from '@dyn/cnv-dtif';
import type { Canvas } from '@dyn/cnv-svg-builder';

import { useOutputEvent } from './use-output-event';

export function useInteractionTool(canvas: Canvas): CNV.InteractionTool {
	const [interactionTool, setInteractionTool] = React.useState<CNV.InteractionTool>({
		type: 'Select'
	});
	useOutputEvent(
		canvas,
		'InteractionToolChange',
		(event) => {
			setInteractionTool(event.interactionTool);
		},
		[]
	);
	return interactionTool;
}
