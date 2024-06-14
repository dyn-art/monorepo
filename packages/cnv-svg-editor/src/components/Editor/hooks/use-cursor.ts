import React from 'react';
import type { CNV } from '@dyn/cnv-dtif';
import type { Canvas } from '@dyn/cnv-svg-builder';

import { useOutputEvent } from './use-output-event';

export function useCursor(canvas?: Canvas): CNV.Cursor {
	const [cursor, setCursor] = React.useState<CNV.Cursor>({
		type: 'Default'
	});
	useOutputEvent(
		canvas,
		'CursorChange',
		(event) => {
			setCursor(event.cursor);
		},
		[]
	);
	return cursor;
}
