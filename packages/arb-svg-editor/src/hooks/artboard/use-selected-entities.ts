import React from 'react';
import type { ARB } from '@dyn/arb-dtif';
import type { Artboard } from '@dyn/arb-svg-builder';

import { useOutputEvent } from './use-output-event';

export function useSelectedEntities(artboart: Artboard): ARB.Entity[] {
	const [selectedEntities, setSelectedEntities] = React.useState<ARB.Entity[]>([]);
	useOutputEvent(
		artboart,
		'SelectionChange',
		(event) => {
			setSelectedEntities(event.selectedEntities);
		},
		[]
	);
	return selectedEntities;
}
