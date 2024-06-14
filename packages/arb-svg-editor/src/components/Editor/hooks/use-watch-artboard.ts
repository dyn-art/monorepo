import React from 'react';
import { flushSync } from 'react-dom';
import type { ARB } from '@dyn/arb-dtif';
import type { Artboard } from '@dyn/arb-svg-builder';

import { useOutputEvent } from './use-output-event';

export function useWatchArtboard(canvas: Artboard, flush = false): Artboard {
	const [_, setArtboardContent] = React.useState<ARB.ArtboardChangeOutputEvent | null>(null);
	useOutputEvent(
		canvas,
		'ArtboardChange',
		(event) => {
			if (flush) {
				// TODO: Validate that flushSync() is a good idea here
				// setTimeout(() => {
				flushSync(() => {
					setArtboardContent(event);
				});
				// });
			} else {
				setArtboardContent(event);
			}
		},
		[]
	);
	return canvas;
}
