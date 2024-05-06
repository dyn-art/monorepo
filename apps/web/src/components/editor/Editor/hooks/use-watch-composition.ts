import React from 'react';
import { flushSync } from 'react-dom';
import type { COMP } from '@dyn/comp-dtif';
import type { Composition } from '@dyn/comp-svg-builder';

import { useOutputEvent } from './use-output-event';

export function useWatchComposition(composition: Composition, flush = false): Composition {
	const [_, setCompositionContent] = React.useState<COMP.CompositionChangeOutputEvent | null>(null);
	useOutputEvent(
		composition,
		'CompositionChange',
		(event) => {
			if (flush) {
				// TODO: Validate that flushSync() is a good idea here
				// setTimeout(() => {
				flushSync(() => {
					setCompositionContent(event);
				});
				// });
			} else {
				setCompositionContent(event);
			}
		},
		[]
	);
	return composition;
}
