import React from 'react';
import type { CNV } from '@dyn/cnv-dtif';
import type { Canvas } from '@dyn/cnv-svg-builder';

import { useOutputEvent } from './use-output-event';

export function useSelectedEntities(canvas: Canvas): CNV.Entity[] {
	const [selectedEntities, setSelectedEntities] = React.useState<CNV.Entity[]>([]);
	useOutputEvent(
		canvas,
		'SelectionChange',
		(event) => {
			setSelectedEntities(event.selectedEntities);
		},
		[]
	);
	return selectedEntities;
}
