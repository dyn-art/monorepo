import React from 'react';
import type { COMP } from '@dyn/dtif-comp';
import type { Composition } from '@dyn/svg-comp';

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
