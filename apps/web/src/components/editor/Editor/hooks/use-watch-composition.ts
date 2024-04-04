import React from 'react';
import type { COMP } from '@dyn/dtif-comp';
import type { Composition } from '@dyn/svg-comp';

import { useOutputEvent } from './use-output-event';

export function useWatchComposition(composition: Composition): Composition {
	const [_, setCompositionContent] = React.useState<COMP.CompositionChangeOutputEvent | null>(null);
	useOutputEvent(
		composition,
		'CompositionChange',
		(event) => {
			setCompositionContent(event);
		},
		[]
	);
	return composition;
}
