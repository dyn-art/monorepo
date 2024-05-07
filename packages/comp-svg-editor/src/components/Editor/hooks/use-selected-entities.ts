import React from 'react';
import type { COMP } from '@dyn/comp-dtif';
import type { Composition } from '@dyn/comp-svg-builder';

import { useOutputEvent } from './use-output-event';

export function useSelectedEntities(composition: Composition): COMP.Entity[] {
	const [selectedEntities, setSelectedEntities] = React.useState<COMP.Entity[]>([]);
	useOutputEvent(
		composition,
		'SelectionChange',
		(event) => {
			setSelectedEntities(event.selectedEntities);
		},
		[]
	);
	return selectedEntities;
}
