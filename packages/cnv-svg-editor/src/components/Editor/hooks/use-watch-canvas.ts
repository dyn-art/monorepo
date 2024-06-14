import React from 'react';
import { flushSync } from 'react-dom';
import type { CNV } from '@dyn/cnv-dtif';
import type { Canvas } from '@dyn/cnv-svg-builder';

import { useOutputEvent } from './use-output-event';

export function useWatchCanvas(canvas: Canvas, flush = false): Canvas {
	const [_, setCanvasContent] = React.useState<CNV.CanvasChangeOutputEvent | null>(null);
	useOutputEvent(
		canvas,
		'CanvasChange',
		(event) => {
			if (flush) {
				// TODO: Validate that flushSync() is a good idea here
				// setTimeout(() => {
				flushSync(() => {
					setCanvasContent(event);
				});
				// });
			} else {
				setCanvasContent(event);
			}
		},
		[]
	);
	return canvas;
}
